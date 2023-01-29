pub mod tcp_server;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::ops::Deref;
use actix_rt::net::TcpStream;

trait IncomingHandler {
    fn handle(&self, data: [u8; 100]);
}

fn listen_on(port: u32, incoming_handler: &impl IncomingHandler) {
    let conn_listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("Running on port {port}");
    for stream in conn_listener.incoming() {
        let mut stream  = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 100];
        stream.read(&mut buffer).unwrap();
        println!("Received from client {}", String::from_utf8_lossy(&buffer));
        incoming_handler.handle(buffer)
    }
}