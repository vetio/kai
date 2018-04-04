// Messages for connecting to the server

#[derive(Clone, PartialEq, Message)]
pub struct ConnectionRequest {
    #[prost(enumeration = "connection_request::Type", tag = "1")]
    pub type_: i32,
    #[prost(string, tag = "2")]
    pub client_name: String,
    #[prost(bytes, tag = "3")]
    pub client_identifier: Vec<u8>,
}
pub mod connection_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
    pub enum Type {
        Rpc = 0,
        Stream = 1,
    }
}
#[derive(Clone, PartialEq, Message)]
pub struct ConnectionResponse {
    #[prost(enumeration = "connection_response::Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub message: String,
    #[prost(bytes, tag = "3")]
    pub client_identifier: Vec<u8>,
}
pub mod connection_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
    pub enum Status {
        Ok = 0,
        MalformedMessage = 1,
        Timeout = 2,
        WrongType = 3,
    }
}
// Messages for calling remote procedures

#[derive(Clone, PartialEq, Message)]
pub struct Request {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::std::vec::Vec<ProcedureCall>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ProcedureCall {
    #[prost(string, tag = "1")]
    pub service: String,
    #[prost(string, tag = "2")]
    pub procedure: String,
    #[prost(uint32, tag = "4")]
    pub service_id: u32,
    #[prost(uint32, tag = "5")]
    pub procedure_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub arguments: ::std::vec::Vec<Argument>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Argument {
    #[prost(uint32, tag = "1")]
    pub position: u32,
    #[prost(bytes, tag = "2")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Response {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<ProcedureResult>,
}
#[derive(Clone, PartialEq, Message)]
pub struct ProcedureResult {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(bytes, tag = "2")]
    pub value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub service: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub description: String,
    #[prost(string, tag = "4")]
    pub stack_trace: String,
}
// Messages for receiving stream updates

#[derive(Clone, PartialEq, Message)]
pub struct StreamUpdate {
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<StreamResult>,
}
#[derive(Clone, PartialEq, Message)]
pub struct StreamResult {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub result: ::std::option::Option<ProcedureResult>,
}
// Messages for receiving information about the server

#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Services {
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Service {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, repeated, tag = "2")]
    pub procedures: ::std::vec::Vec<Procedure>,
    #[prost(message, repeated, tag = "3")]
    pub classes: ::std::vec::Vec<Class>,
    #[prost(message, repeated, tag = "4")]
    pub enumerations: ::std::vec::Vec<Enumeration>,
    #[prost(message, repeated, tag = "5")]
    pub exceptions: ::std::vec::Vec<Exception>,
    #[prost(string, tag = "6")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Procedure {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::std::vec::Vec<Parameter>,
    #[prost(message, optional, tag = "3")]
    pub return_type: ::std::option::Option<Type>,
    #[prost(bool, tag = "4")]
    pub return_is_nullable: bool,
    #[prost(string, tag = "5")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Parameter {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, optional, tag = "2")]
    pub type_: ::std::option::Option<Type>,
    #[prost(bytes, tag = "3")]
    pub default_value: Vec<u8>,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Class {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Enumeration {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, repeated, tag = "2")]
    pub values: ::std::vec::Vec<EnumerationValue>,
    #[prost(string, tag = "3")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct EnumerationValue {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(int32, tag = "2")]
    pub value: i32,
    #[prost(string, tag = "3")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Exception {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub documentation: String,
}
#[derive(Clone, PartialEq, Message, Serialize)]
pub struct Type {
    #[prost(enumeration = "type_::TypeCode", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub service: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(message, repeated, tag = "4")]
    pub types: ::std::vec::Vec<Type>,
}
pub mod type_ {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
    pub enum TypeCode {
        None = 0,
        /// Values
        Double = 1,
        Float = 2,
        Sint32 = 3,
        Sint64 = 4,
        Uint32 = 5,
        Uint64 = 6,
        Bool = 7,
        String = 8,
        Bytes = 9,
        /// Objects
        Class = 100,
        Enumeration = 101,
        /// Messages
        Event = 200,
        ProcedureCall = 201,
        Stream = 202,
        Status = 203,
        Services = 204,
        /// Collections
        Tuple = 300,
        List = 301,
        Set = 302,
        Dictionary = 303,
    }
}
// Collection data structures

#[derive(Clone, PartialEq, Message)]
pub struct Tuple {
    #[prost(bytes, repeated, tag = "1")]
    pub items: ::std::vec::Vec<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct List {
    #[prost(bytes, repeated, tag = "1")]
    pub items: ::std::vec::Vec<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Set {
    #[prost(bytes, repeated, tag = "1")]
    pub items: ::std::vec::Vec<Vec<u8>>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Dictionary {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<DictionaryEntry>,
}
#[derive(Clone, PartialEq, Message)]
pub struct DictionaryEntry {
    #[prost(bytes, tag = "1")]
    pub key: Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub value: Vec<u8>,
}
// Aggregate data structures

#[derive(Clone, PartialEq, Message)]
pub struct Stream {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, Message)]
pub struct Event {
    #[prost(message, optional, tag = "1")]
    pub stream: ::std::option::Option<Stream>,
}
#[derive(Clone, PartialEq, Message)]
pub struct Status {
    #[prost(string, tag = "1")]
    pub version: String,
    #[prost(uint64, tag = "2")]
    pub bytes_read: u64,
    #[prost(uint64, tag = "3")]
    pub bytes_written: u64,
    #[prost(float, tag = "4")]
    pub bytes_read_rate: f32,
    #[prost(float, tag = "5")]
    pub bytes_written_rate: f32,
    #[prost(uint64, tag = "6")]
    pub rpcs_executed: u64,
    #[prost(float, tag = "7")]
    pub rpc_rate: f32,
    #[prost(bool, tag = "8")]
    pub one_rpc_per_update: bool,
    #[prost(uint32, tag = "9")]
    pub max_time_per_update: u32,
    #[prost(bool, tag = "10")]
    pub adaptive_rate_control: bool,
    #[prost(bool, tag = "11")]
    pub blocking_recv: bool,
    #[prost(uint32, tag = "12")]
    pub recv_timeout: u32,
    #[prost(float, tag = "13")]
    pub time_per_rpc_update: f32,
    #[prost(float, tag = "14")]
    pub poll_time_per_rpc_update: f32,
    #[prost(float, tag = "15")]
    pub exec_time_per_rpc_update: f32,
    #[prost(uint32, tag = "16")]
    pub stream_rpcs: u32,
    #[prost(uint64, tag = "17")]
    pub stream_rpcs_executed: u64,
    #[prost(float, tag = "18")]
    pub stream_rpc_rate: f32,
    #[prost(float, tag = "19")]
    pub time_per_stream_update: f32,
}
// Multiplexed request messages

#[derive(Clone, PartialEq, Message)]
pub struct MultiplexedRequest {
    #[prost(message, optional, tag = "1")]
    pub connection_request: ::std::option::Option<ConnectionRequest>,
    #[prost(message, optional, tag = "2")]
    pub request: ::std::option::Option<Request>,
}
#[derive(Clone, PartialEq, Message)]
pub struct MultiplexedResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::std::option::Option<Response>,
    #[prost(message, optional, tag = "2")]
    pub stream_update: ::std::option::Option<StreamUpdate>,
}
