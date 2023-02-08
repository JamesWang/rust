use std::io;
use std::io::{IoSlice, Read, Write};
use std::net::TcpStream;
use std::str::Split;
use actix_rt::spawn;
use actix_web::body::to_bytes;
use crate::admin::music_paths;
use super::tcp_server::IncomingHandler;

const MUSIC_PATH: &str = "V:\\MusicPhotos\\music";

#[derive(Debug, Clone)]
pub struct MusicInfo {
    music_path: String
}

impl  MusicInfo {
    fn split_data(cmd_full: &str) -> (&str, Option<&str>) {
        let split: Vec<&str> = cmd_full.split(" ").collect();
        (split[0], if split.len() > 1 {Some(split[1])} else {None})
    }

    fn list(&self, mut stream: &TcpStream) -> () {
        println!("Handling /List");
        let paths = &music_paths(&self.music_path.clone());
        let result = paths.join("\n");
        stream.write(result.as_bytes()).unwrap();
        stream.flush().unwrap();
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
    fn new(music_path: String) -> MusicInfo {
        MusicInfo { music_path }
    }
    fn handle(&self, mut stream: &TcpStream) {
        let mut buffer = [0; 100];
        let read_cnt = stream.read(&mut buffer).unwrap();

        let cmd_full: &String = &String::from_utf8(buffer[0..read_cnt].to_owned()).unwrap().trim().to_string();
        println!("Received from client {read_cnt} {cmd_full}");
        let (cmd, arg) = MusicInfo::split_data(cmd_full);
        println!("before switch {cmd}");
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