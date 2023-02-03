use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;

pub(crate) trait IncomingHandler {
    fn handle(&self, stream: &TcpStream);
}

fn listen_on(port: u32, incoming_handler: &impl IncomingHandler) {
    let conn_listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    println!("Running on port {port}");
    for stream in conn_listener.incoming() {
        let mut stream  = &stream.unwrap();
        println!("Connection established");
        incoming_handler.handle(stream)
    }
}