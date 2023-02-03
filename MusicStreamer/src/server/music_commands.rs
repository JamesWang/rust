use std::io;
use std::io::{IoSlice, Read, Write};
use std::net::TcpStream;
use std::str::Split;
use actix_rt::spawn;
use actix_web::body::to_bytes;
use crate::admin::music_paths;
use super::tcp_server::IncomingHandler;

const MUSIC_PATH: &str = "V:\\MusicPhotos\\music";

struct MusicInfo {
    music_path: String
}

impl  MusicInfo {
    fn split_data(cmd_full: &str) -> (&str, Option<&str>) {
        let split: Vec<&str> = cmd_full.split(" ").collect();
        (split[0], if split.len() > 1 {Some(split[1])} else {None})
    }

    fn list(&self, mut stream: &TcpStream) -> () {
        let paths = &music_paths(&self.music_path.clone());
        let result = paths.join("\n");
        stream.write(result.as_bytes()).expect("TODO: panic message");
        return 
    }

    fn play(&self){
    }

    fn pause(&self){
    }

    fn schedule(&self, tracks: Vec<String>, mut stream: &TcpStream){

    }
}

impl IncomingHandler for MusicInfo {

    fn handle(&self, mut stream: &TcpStream) {
        let mut buffer = [0; 100];
        stream.read(&mut buffer).unwrap();

        let cmd_full: &String = &String::from_utf8(buffer.to_vec()).unwrap();
        println!("Received from client {}", cmd_full);
        let (cmd, arg) = MusicInfo::split_data(cmd_full);
        match cmd {
            "/list" => self.list(stream),
            "/play" => self.play(),
            "/pause" => self.pause(),
            "schedule" => self.schedule(Vec::new(), stream),
            _ => {}
        };
        ()
    }
}