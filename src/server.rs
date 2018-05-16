use std::fmt::{Debug, Display};
use std::io;

use futures::prelude::*;

use connection::RpcConnection;
use schema;

#[derive(Debug)]
pub struct Server<C> {
    connection: C,
}

impl<C> Server<C> {
    pub fn new(connection: C) -> Self {
        Server { connection }
    }

    pub fn into_inner(self) -> C {
        self.connection
    }
}

impl<C: RpcConnection> Server<C> {
    #[async]
    pub fn invoke<P: ProcedureCall>(
        self,
        p: P,
    ) -> Result<(P::Result, Self), ProcedureCallError<P, C>> {
        let request = schema::Request {
            calls: vec![p.into()],
        };

        let (response, c): (schema::Response, C) = await!(self.connection.call(request))?;
        if let Some(e) = response.error {
            return Err(ProcedureCallError::Request(e, Self::new(c)));
        }

        let result = match response.results.into_iter().next() {
            Some(result) => result,
            None => return Err(ProcedureCallError::NoResult(Self::new(c))),
        };

        if let Some(e) = result.error {
            return Err(ProcedureCallError::Procedure(e.into(), Self::new(c)));
        }

        let result = result.value;

        use self::FromProcedureResult;
        let result = match P::Result::try_from(result) {
            Ok(v) => v,
            Err(e) => return Err(ProcedureCallError::Decode(e, Self::new(c))),
        };

        Ok((result, Self::new(c)))
    }
}

#[derive(Debug)]
pub enum ProcedureCallError<P: ProcedureCall, C> {
    Connection(io::Error),
    Procedure(P::Error, Server<C>),
    NoResult(Server<C>),
    Request(schema::Error, Server<C>),
    Decode(<P::Result as FromProcedureResult>::Error, Server<C>),
}

impl<P: ProcedureCall, C> ::std::fmt::Display for ProcedureCallError<P, C> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            ProcedureCallError::Connection(ref e) => write!(f, "Connection Error: {}", e),
            ProcedureCallError::Procedure(ref e, _) => write!(f, "Procedure Error: {}", e),
            ProcedureCallError::NoResult(_) => write!(f, "No result for procedure call"),
            ProcedureCallError::Request(ref e, _) => write!(f, "Request Error: {}", e),
            ProcedureCallError::Decode(ref e, _) => write!(f, "Decode Error: {:?}", e),
        }
    }
}

impl<P, C> ::failure::Fail for ProcedureCallError<P, C>
where
    P: ProcedureCall + Debug,
    P::Result: Debug,
    <P::Result as FromProcedureResult>::Error: Debug + Send + Sync,
    C: Debug + Send + Sync + 'static,
{
    fn cause(&self) -> Option<&::failure::Fail> {
        match *self {
            ProcedureCallError::Connection(ref e) => Some(e),
            ProcedureCallError::Procedure(ref e, _) => Some(e),
            _ => None,
        }
    }
}

impl<P, C> From<io::Error> for ProcedureCallError<P, C>
where
    P: ProcedureCall,
{
    fn from(e: io::Error) -> Self {
        ProcedureCallError::Connection(e)
    }
}

pub trait ProcedureCall: Into<schema::ProcedureCall> + 'static {
    type Result: FromProcedureResult;
    type Error: ::failure::Fail
        + From<schema::Error>
        + From<<Self::Result as FromProcedureResult>::Error>;
}

#[derive(Debug)]
pub struct KrpcGetStatus;

impl ProcedureCall for KrpcGetStatus {
    type Result = schema::Status;
    type Error = SimpleResultError;
}

impl From<KrpcGetStatus> for schema::ProcedureCall {
    fn from(_: KrpcGetStatus) -> Self {
        schema::ProcedureCall {
            service: "KRPC".to_string(),
            procedure: "GetStatus".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Fail)]
pub enum SimpleResultError {
    #[fail(display = "Server returned error: {}", _0)]
    Server(schema::Error),
    #[fail(display = "Error decoding results: {}", _0)]
    Decode(::prost::DecodeError),
}

impl From<schema::Error> for SimpleResultError {
    fn from(err: schema::Error) -> Self {
        SimpleResultError::Server(err)
    }
}

impl From<::prost::DecodeError> for SimpleResultError {
    fn from(e: ::prost::DecodeError) -> Self {
        SimpleResultError::Decode(e)
    }
}

pub trait FromProcedureResult
where
    Self: ::std::marker::Sized,
{
    type Error: Debug;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error>;
}

impl<M: ::prost::Message + Default> FromProcedureResult for M {
    type Error = ::prost::DecodeError;
    fn try_from(value: Vec<u8>) -> Result<Self, ::prost::DecodeError> {
        Self::decode(value)
    }
}

impl ::failure::Fail for schema::Error {}

impl Display for schema::Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_fmt(format_args!("{}", self.description))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests::run_test;

    use proptest::prelude::*;

    use connection::RpcConnection;

    static MOCK_SERVICE: &str = "mock-service";
    static MOCK_PROCEDURE: &str = "mock-procedure";

    #[test]
    fn test_server_into_inner() {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        struct MockConnection(u32);

        let connection = MockConnection(42);

        let server = Server::new(connection);

        let unwrapped_connection = server.into_inner();

        assert_eq!(connection, unwrapped_connection);
    }

    #[test]
    fn test_echo() {
        run_test(
            &any::<u32>(),
            |v| {
                #[derive(Debug)]
                struct MockConnection;
                impl RpcConnection for MockConnection {
                    fn call(
                        self,
                        r: ::schema::Request,
                    ) -> Box<
                        Future<Item = (::schema::Response, Self), Error = ::std::io::Error>
                            + ::std::marker::Send,
                    > {
                        let mut call = extract_call(r);
                        assert_eq!(1, call.arguments.len());

                        let argument = call.arguments.pop().unwrap();
                        assert_eq!(0, argument.position);

                        use prost::Message;

                        let arg = u32::decode(argument.value).unwrap();

                        let mut encoded = Vec::new();
                        arg.encode(&mut encoded).unwrap();

                        Box::new(::futures::future::ok((
                            ::schema::Response {
                                error: None,
                                results: vec![::schema::ProcedureResult {
                                    error: None,
                                    value: encoded,
                                }],
                            },
                            self,
                        )))
                    }
                }

                #[derive(Debug, Eq, PartialEq)]
                struct MockRequest {
                    data: u32,
                }

                impl ProcedureCall for MockRequest {
                    type Result = u32;
                    type Error = SimpleResultError;
                }

                impl From<MockRequest> for ::schema::ProcedureCall {
                    fn from(r: MockRequest) -> Self {
                        use prost::Message;

                        let mut buffer = Vec::new();

                        r.data.encode(&mut buffer).unwrap();

                        ::schema::ProcedureCall {
                            service: String::from(MOCK_SERVICE),
                            procedure: String::from(MOCK_PROCEDURE),
                            arguments: vec![::schema::Argument {
                                position: 0,
                                value: buffer,
                            }],
                            ..Default::default()
                        }
                    }
                }

                let request = MockRequest { data: *v };

                let server = Server::new(MockConnection);

                let (result, _) = server.invoke(request).wait().unwrap();

                prop_assert_eq!(*v, result);

                Ok(())
            },
            file!(),
        ).unwrap();
    }

    fn extract_call(mut request: ::schema::Request) -> ::schema::ProcedureCall {
        assert_eq!(1, request.calls.len());
        let call = request.calls.pop().unwrap();

        assert_eq!(MOCK_SERVICE, call.service);
        assert_eq!(MOCK_PROCEDURE, call.procedure);

        call
    }
}
