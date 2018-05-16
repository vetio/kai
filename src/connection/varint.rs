use bytes::BytesMut;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum DecodedVarint {
    Ok { value: u32, bytes: usize },
    NotEnough,
    Invalid,
}

pub(crate) fn try_decode_varint(mut buf: &[u8]) -> DecodedVarint {
    let mut bytes = 0;
    let mut value = 0;

    loop {
        if bytes >= 5 {
            return DecodedVarint::Invalid;
        }

        if buf.is_empty() {
            return DecodedVarint::NotEnough;
        }

        let b = buf[0];
        buf = &buf[1..];

        let mut v = (b & 0x7F) as u32;

        v <<= 7 * bytes;
        value |= v;
        bytes += 1;

        if b <= 0x7F {
            return DecodedVarint::Ok { value, bytes };
        }
    }
}

pub(crate) fn encode_varint(buf: &mut BytesMut, mut val: u32) {
    use bytes::BufMut;
    loop {
        let mut b = (val & 0x7F) as u8;

        val >>= 7;
        if val > 0 {
            b |= 0x80;
        }
        buf.put_u8(b);
        if val == 0 {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use tests::run_test;

    #[test]
    fn test_try_decode_varint() {
        assert_eq!(try_decode_varint(&[]), DecodedVarint::NotEnough);
        assert_eq!(
            try_decode_varint(&[0]),
            DecodedVarint::Ok { value: 0, bytes: 1 }
        );
        assert_eq!(
            try_decode_varint(&[0b01111111]),
            DecodedVarint::Ok {
                value: 0b01111111,
                bytes: 1,
            }
        );
        assert_eq!(try_decode_varint(&[0b10000001]), DecodedVarint::NotEnough);
        assert_eq!(
            try_decode_varint(&[0b10000000, 0b00000001]),
            DecodedVarint::Ok {
                value: 0b10000000,
                bytes: 2,
            }
        );
        assert_eq!(
            try_decode_varint(&[0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b00001111]),
            DecodedVarint::Ok {
                value: 0xFFFFFFFF,
                bytes: 5,
            }
        );
        assert_eq!(
            try_decode_varint(&[0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b01111111]),
            DecodedVarint::Ok {
                value: 0xFFFFFFFF,
                bytes: 5,
            }
        );
        assert_eq!(
            try_decode_varint(&[0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0,]),
            DecodedVarint::Invalid
        );
        assert_eq!(
            try_decode_varint(&[0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111]),
            DecodedVarint::Invalid
        );
    }

    #[test]
    fn test_varint_framed_roundtrip() {
        // ref v in any::<Vec<u8>>()
        run_test(&any::<u32>(), |&value| {
                let mut buf = BytesMut::new();
                encode_varint(&mut buf, value);
                let bytes = buf.len();
                let decoded = try_decode_varint(&buf);

                let expected = DecodedVarint::Ok { value, bytes };
                prop_assert_eq!(expected, decoded, "i = {}, buf = {:?}", value, buf);

                Ok(())
            }, file!())
            .unwrap();
    }
}
