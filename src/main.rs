#![feature(proc_macro, generators, pin)]

extern crate bytes;
#[macro_use]
extern crate futures_await as futures;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate tokio_io;

#[cfg(test)]
#[macro_use]
extern crate proptest;

use futures::prelude::*;
use tokio::io;

pub mod connection;
mod schema;

#[async]
fn run() -> io::Result<()> {
    use std::net::SocketAddr;
    use connection::RpcConnection;
    use prost::Message;

    let addr = "127.0.0.1:50000".parse::<SocketAddr>().unwrap();

    let tcp = await!(::tokio::net::TcpStream::connect(&addr))?;

    let c = await!(connection::TokioConnection::initialize(tcp))?;
    println!("Connection established");

    let request = schema::Request {
        calls: vec![
            schema::ProcedureCall {
                service: "KRPC".to_owned(),
                procedure: "GetStatus".to_owned(),
                ..Default::default()
            },
        ],
    };

    println!("Sending request: {:#?}", request);
    let (response, c) = await!(c.call(request))?;
    println!("Response: {:#?}", response);

    let status = schema::Status::decode(&response.results[0].value);
    println!("Status: {:#?}", status);

    let request = schema::Request {
        calls: vec![
            schema::ProcedureCall {
                service: "KRPC".to_owned(),
                procedure: "GetServices".to_owned(),
                ..Default::default()
            },
        ],
    };

    println!("Sending request: {:#?}", request);
    let (response, c) = await!(c.call(request))?;

    let services = schema::Services::decode(&response.results[0].value).unwrap();

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("services.json")?;

    writeln!(file, "{}", serde_json::to_string(&services).unwrap()).unwrap();

    drop(c);
    println!("Connection closed");

    Ok(())
}

fn main() {
    tokio::run(run().map_err(|e| println!("Error: {:#?}", e)));
}
