use std::io::{Read, Write};
use std::net::{Incoming, TcpListener, TcpStream};
use std::ops::Deref;
use crate::server::music_commands::MusicInfo;
use threadpool::ThreadPool;

pub(crate) trait IncomingHandler {
    fn new(t: String) -> Self;
    fn handle(&self, stream: &TcpStream);
}

pub fn listen_on(port: u32, incoming_handler: MusicInfo) {
    let pool = &ThreadPool::new(4);
    let conn_listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    println!("Running on port {port}");
    incoming_handler.clone();
    for stream in conn_listener.incoming() {
        println!("Connection established");
        let in_handler = incoming_handler.clone();
        pool.execute(move|| in_handler.handle(&stream.unwrap()));
        println!("Handling next connection")
    }
}