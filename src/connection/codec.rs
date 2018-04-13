use std::io;

use bytes::BytesMut;
use tokio_io::codec;

use super::varint::*;

pub struct VarintFramedCodec;

impl codec::Encoder for VarintFramedCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), io::Error> {
        use bytes::BufMut;

        let len = item.len();
        if len > u32::max_value() as usize {
            panic!("message too long");
        }
        dst.reserve(len + 5);
        super::varint::encode_varint(dst, len as u32);
        dst.put(item);
        Ok(())
    }
}

impl codec::Decoder for VarintFramedCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
        match try_decode_varint(&*src) {
            DecodedVarint::NotEnough => Ok(None),
            DecodedVarint::Invalid => panic!("ERROR"),
            DecodedVarint::Ok { value, bytes } => {
                let value = value as usize;
                let total_len = value + bytes;
                if src.len() < total_len {
                    Ok(None)
                } else {
                    src.split_to(bytes);
                    Ok(Some(src.split_to(value)))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::TestRunner;

    #[test]
    fn test_varint_framed_roundtrip() {
        let mut runner = TestRunner::default();
        runner.set_source_file(::std::path::Path::new(file!()));
        runner
            .run(&any::<Vec<u8>>(), |v| {
                use tokio_io::codec::{Decoder, Encoder};

                let mut codec = VarintFramedCodec;
                let mut buffer = BytesMut::new();

                codec.encode(v.clone(), &mut buffer).unwrap();

                // Incomplete
                let len = buffer.len();
                let tail = buffer.split_off(len / 2);
                let result = codec.decode(&mut buffer)?;
                assert!(result.is_none());

                // Complete
                buffer.unsplit(tail);
                let result = codec.decode(&mut buffer)?.unwrap();
                assert_eq!(v, &result.into_iter().collect::<Vec<u8>>());

                Ok(())
            })
            .unwrap();
    }
}
