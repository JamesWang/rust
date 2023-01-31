use std::str::Split;
use actix_rt::spawn;
use super::tcp_server::IncomingHandler;

impl MusicCommands {
    fn split_data(cmd_full: &str) -> (&str, Option<&str>) {
        split: Vec<&str> = cmd_full.split(" ").collect();
        (split[0], if split.len() > 1 {Some(split[1])} else {None})
    }

    fn list(&self) -> Vec<String> {
        Vec::new()
    }

    fn play(&self){
    }

    fn pause(&self){
    }

    fn schedule(&self, tracks: Vec<String>){

    }
}

impl IncomingHandler for MusicCommands {


    fn handle(&self, data: [u8; 100]) {
        let cmd_full: &str = str::from_utf8(&data);
        let (cmd, arg) = split_data(cmd_full);
        match cmd {
            "/list" => "",
            "/play" => "",
            "/pause" => "",
            "schedule" => ""
        }
        ()
    }
}