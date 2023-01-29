use std::any::Any;
use std::collections::HashMap;
use std::iter::Map;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use actix_web::HttpResponse;
use atomic_counter::AtomicCounter;

enum JokeBoxState {
    Playing,
    Paused
}

enum JokeBoxCommand {
    List,
    Play,
    Pause,
    Schedule
}

pub struct JokeBoxData {
    command: JokeBoxCommand,
    data: Vec<String>
}

pub struct JokeBoxContext {
    current_status: JokeBoxState,
    streamers: HashMap<usize, *mut HttpResponse>,
    play_list: Vec<String>,
    position_in_file: usize
}

impl JokeBoxContext {
    fn register_listener(&mut self, key: usize, listener: &mut HttpResponse) -> Option<*mut HttpResponse> {
        self.streamers.insert(key, listener)
    }

    fn unregister(&mut self, key: usize) -> Option<*mut HttpResponse> {
        self.streamers.remove(&key)
    }
}

pub struct Counter {
    pub counter: dyn AtomicCounter<PrimitiveType=usize>
}

impl Counter {
    fn inc(&self) -> usize {
        self.counter.inc().into()
    }

    fn get_and_inc(&self) -> usize {
        let curr = self.counter.get();
        self.counter.inc();
        return curr
    }
}