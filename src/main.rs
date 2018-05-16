#![feature(proc_macro, generators, pin)]
#![feature(proc_macro_non_items)]

#[macro_use]
extern crate failure;

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

pub mod connection;
mod schema;
mod server;

#[cfg(test)]
mod tests;

#[async]
fn run() -> Result<(), failure::Error> {
    use std::net::SocketAddr;

    let addr = "127.0.0.1:50000".parse::<SocketAddr>().unwrap();

    let tcp = await!(::tokio::net::TcpStream::connect(&addr))?;

    let c = await!(connection::TokioConnection::initialize(tcp))?;

    let server = server::Server::new(c);
    println!("Connection established");

    let call = server::KrpcGetStatus;

    let (response, _) = await!(server.invoke(call))?;

    println!("Response: {:?}", response);

    println!("Connection closed");

    Ok(())
}

fn main() {
    tokio::run(run().map_err(|e| println!("Error: {:#?}", e)));
}
