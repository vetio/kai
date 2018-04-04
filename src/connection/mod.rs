mod varint;
mod codec;

use std::io;

use futures::prelude::*;
use tokio_io::{AsyncRead, AsyncWrite, codec::Framed};

use schema;

trait RpcConnection
where
    Self: Sized + Sink<SinkItem = schema::Request> + Stream<Item = schema::Response>,
{
}

pub struct TokioConnection<A> {
    inner: Framed<A, codec::VarintFramedCodec>,
    handshake_response: schema::ConnectionResponse,
}

impl<A> TokioConnection<A>
where
    A: AsyncRead + AsyncWrite + 'static,
{
    #[async]
    pub fn initialize(io: A) -> io::Result<Self> {
        let framed = io.framed(codec::VarintFramedCodec);

        await!(do_handshake(framed))
    }
}

#[async]
fn do_handshake<A>(t: Framed<A, codec::VarintFramedCodec>) -> io::Result<TokioConnection<A>>
where
    A: AsyncRead + AsyncWrite + 'static,
{
    use prost::Message;
    use futures::Sink;

    let request = schema::ConnectionRequest {
        type_: schema::connection_request::Type::Rpc.into(),
        client_name: "Test".to_owned(),
        client_identifier: Vec::new(),
    };

    let mut buf = Vec::new();

    request.encode(&mut buf)?;

    use bytes::buf::IntoBuf;
    let t = await!(t.send(buf))?;
    let (response, t) = await!(t.into_future().map_err(|(e, _)| e))?;
    let response =
        response.ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No connection response"))?;
    let response = schema::ConnectionResponse::decode(&mut response.into_buf())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(TokioConnection {
        inner: t,
        handshake_response: response,
    })
}

impl<A> Sink for TokioConnection<A>
where
    A: AsyncRead + AsyncWrite,
{
    type SinkItem = schema::Request;
    type SinkError = io::Error;

    fn start_send(
        &mut self,
        item: schema::Request,
    ) -> Result<AsyncSink<schema::Request>, io::Error> {
        use prost::Message;

        let mut buf = Vec::new();
        item.encode(&mut buf)?;

        let res = match self.inner.start_send(buf)? {
            AsyncSink::Ready => AsyncSink::Ready,
            AsyncSink::NotReady(_) => AsyncSink::NotReady(item),
        };
        Ok(res)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, io::Error> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Result<Async<()>, io::Error> {
        self.inner.close()
    }
}

impl<A> Stream for TokioConnection<A>
where
    A: AsyncRead + AsyncWrite,
{
    type Item = schema::Response;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<schema::Response>>, io::Error> {
        use prost::Message;

        let inner_item = match try_ready!(self.inner.poll()) {
            Some(v) => v,
            None => return Ok(Async::Ready(None)),
        };

        let response = schema::Response::decode(inner_item)?;
        Ok(Async::Ready(Some(response)))
    }
}
