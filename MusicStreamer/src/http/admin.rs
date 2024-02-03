use std::fs;
use std::fs::{DirEntry, FileType};
use std::io::Error;
use std::path::PathBuf;
use actix_web::{web, App, get, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::ResourcePath;
use actix_web::http::header::ContentType;
use bytes::Bytes;
use async_stream::{try_stream, AsyncStream};
use std::io::prelude::*;
use std::ptr::addr_of_mut;
use std::sync::Arc;
use actix_web::cookie::time::format_description::well_known::iso8601::Config;
use partial_application::partial;
use serde::Serialize;

pub struct MusicConf {
    pub(crate) music_path: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

fn is_file(de: &DirEntry) -> bool {
    return de.file_type().unwrap().is_file();
}

fn to_filename(e: &DirEntry) -> Option<String> {
    return e.path()
        .file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)));
}

fn extract_filename(e: Option<DirEntry>) -> Option<String> {
    e.filter(is_file).and_then(|e| to_filename(&e))
}

fn as_string<T:Serialize>(data: &T) -> String {
    return serde_json::to_string(data).unwrap()
}

pub fn music_paths(music_path: &String) -> Vec<String> {
    fs::read_dir(music_path)
        .unwrap()
        .filter_map(|e| extract_filename(e.ok()))
        .collect::<Vec<String>>()
}

//#[get("/list")]
async fn list_music(data: web::Data<MusicConf>) -> impl Responder {
    let paths = music_paths(&data.music_path.clone());

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(as_string(&paths));
}

async fn list_music2(music_path: String) -> impl Responder {
    let paths = music_paths(&music_path.to_string());

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(as_string(&paths));
}

fn music_response(x: PathBuf) -> HttpResponse {
    let stream: AsyncStream<Result<Bytes, Error>, _> = try_stream! {
        let mut file = fs::File::open(x)?;
        let mut chunk = [0; 4096];
        loop {
            let n = file.read(&mut chunk[..])?;
            if n == 0 {
                break;
            }
            yield Bytes::copy_from_slice(&chunk[..n]);
        }
    };
    HttpResponse::Ok().content_type("audio/mpeg").streaming(stream)
}

//#[get("/play")]
async fn play(data: web::Data<MusicConf>, req: HttpRequest) -> HttpResponse {
    let music = req.match_info().get("music").unwrap();
    return music_response(PathBuf::from(format!("{}\\{}", data.music_path, music)))
}

#[get("/schedule")]
async fn schedule(req: HttpRequest) -> impl Responder {
    "schedule music to play"
}


pub async fn start_server_at(host_port: &String, music_path: &String) -> std::io::Result<()> {
    let data = web::Data::new(MusicConf{music_path: music_path.clone()});
    HttpServer::new(move|| {
        App::new()
            .app_data(data.clone())
            .service(web::resource("/list")
                .route(web::get().to(list_music)))
            .service(schedule)
            .route("/play/{music}", web::get().to(play))
    }).bind(host_port)?.run().await
}

pub async fn start_server_at2(host_port: &str, music_path: String) -> std::io::Result<()> {
    println!("Http Listening on {host_port}");
    HttpServer::new(move|| {
        let mmp = music_path.clone();
        App::new()
            .service(web::resource("/list")
                .route(web::get().to(move || { list_music2(mmp.to_string()) })))
            .service(schedule)
            .route("/play/{music}", web::get().to(play))
    }).bind(host_port)?.run().await
}

pub async fn start_server_at3(host_port: &str, music_path: String) -> std::io::Result<()> {
    let http_server = move|mp: String| { move|| {
        let mmp = mp.clone();
        App::new()
            .service(web::resource("/list")
                .route(web::get().to(move|| { list_music2(mmp.clone()) })))
            .service(schedule)
            .route("/play/{music}", web::get().to(play))
    }};
    HttpServer::new(http_server(music_path)).bind(host_port)?.run().await
}