use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref AUDIO_LOCK: Mutex<()> = Mutex::new(());
}

const BASEPATH: &str = "/home/realraum/welcomesounds";

#[derive(Debug, Serialize)]
struct Sound {
    name: String,
    path: String,
}

fn play_sound_from_path(filepath: &str) {
    let _lock = AUDIO_LOCK.lock().unwrap();
    Command::new("mplayer")
        .args(&[
            "-really-quiet",
            "-nolirc",
            "-ao",
            "alsa",
            &format!("{}/{}", BASEPATH, filepath),
        ])
        .spawn()
        .expect("Failed to execute mplayer");
}

fn get_sounds_list() -> Vec<Sound> {
    let mut sounds = Vec::new();
    for entry in fs::read_dir(Path::new(BASEPATH)).unwrap() {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                let filepath = entry.path();
                let fname = filepath.strip_prefix(BASEPATH).unwrap_or(&filepath);
                let fname = fname.to_str().unwrap_or("");
                sounds.push(Sound {
                    name: filename.to_string(),
                    path: fname.to_string(),
                });
            }
        }
    }
    sounds
}

async fn sounds_handler(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let sounds = get_sounds_list();
    let response = json!({ "sounds": sounds });
    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

/*
async fn play_sound_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let filename = req.uri().path().trim_start_matches("/play/");
    let filepath = format!("{}/{}", BASEPATH, filename);
    if Path::new(&filepath).exists() {
        let _lock = AUDIO_LOCK.lock().unwrap();
        thread::spawn(move || play_sound_from_path(filename));
        Ok(Response::new(Body::empty()))
    } else {
        let response = "File not found";
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/plain")
            .body(Body::from(response))
            .unwrap())
    }
}
*/

/*
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_svc = make_service_fn(|_conn| {
        let sounds_handler = sounds_handler.clone();
        let play_sound_handler = play_sound_handler.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                match (req.method(), req.uri().path()) {
                    (&hyper::Method::GET, "/api/sounds") => sounds_handler(req),
                    (&hyper::Method::POST, path) if path.starts_with("/api/play/") => {
                        play_sound_handler(req)
                    }
                    _ => {
                        let response = "Not Found";
                        Ok(Response::builder()
                            .status(404)
                            .header("Content-Type", "text/plain")
                            .body(Body::from(response))
                            .unwrap())
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
*/

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:420420".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
