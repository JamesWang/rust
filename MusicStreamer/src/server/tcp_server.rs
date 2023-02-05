use std::io::{Read, Write};
use std::net::{Incoming, TcpListener, TcpStream};
use std::ops::Deref;
use crate::server::music_commands::MusicInfo;

pub(crate) trait IncomingHandler {
    fn new(t: String) -> Self;
    fn handle(&self, stream: &TcpStream);
}

pub fn listen_on(port: u32, incoming_handler: MusicInfo) {
    let conn_listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    println!("Running on port {port}");
    for stream in conn_listener.incoming() {
        println!("Connection established");
        incoming_handler.handle(&stream.unwrap())
    }
}