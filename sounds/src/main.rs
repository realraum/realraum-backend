use axum::{
    extract::Path,
    http::{StatusCode, Uri},
    routing::{any, get},
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::{ServeDir, ServeFile};

use std::{
    fs,
    path::{Path as FsPath, PathBuf},
    process::Command,
    sync::Mutex,
};

lazy_static! {
    static ref AUDIO_LOCK: Mutex<()> = Mutex::new(());
}

const BASE_PATH: &str = "/home/realraum/welcomesounds";

#[derive(Debug, Serialize)]
struct Sound {
    name: String,
    path: String,
}

/// Plays a sound from a given path using `mplayer`
///
/// Spawns a new child process and returns immediately.  
/// Multiple sounds are prevented by using a global lock.
pub fn play_sound_from_path(filepath: &str) {
    let _lock = AUDIO_LOCK.lock().unwrap();
    Command::new("mplayer")
        .args(&[
            "-really-quiet",
            "-nolirc",
            "-ao",
            "alsa",
            &format!("{}/{}", BASE_PATH, filepath),
        ])
        .spawn()
        .expect("Failed to execute mplayer");
}

/// Lists all sounds in the [`BASE_PATH`] directory, returning a [`Vec`] of [`Sound`] structs.
fn get_sounds_list() -> Vec<Sound> {
    let mut sounds = Vec::new();
    for entry in fs::read_dir(FsPath::new(BASE_PATH)).unwrap() {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                let filepath = entry.path();
                let fname = filepath.strip_prefix(BASE_PATH).unwrap_or(&filepath);
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

/// API endpoint for listing all sounds on `/api/sounds`
async fn sounds_handler() -> Json<Value> {
    let sounds = get_sounds_list();
    let response = json!(sounds);
    Json(response)
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

#[derive(Debug, Deserialize)]
struct PlaySoundPayload {
    name: String,
}

async fn handle_play_sound(Path(sound_path): Path<String>) -> Json<Value> {
    dbg!(&sound_path);
    let filepath = format!("{}/{}", BASE_PATH, sound_path);
    dbg!(&filepath);
    if FsPath::new(&filepath).exists() {
        dbg!("exists");
        // let _lock = AUDIO_LOCK.lock().unwrap();
        play_sound_from_path(&sound_path);
        Json(json!({ "status": "ok" }))
    } else {
        dbg!("not exists");
        Json(json!({ "status": "error", "message": "File not found" }))
    }
}

async fn handle_killall_mplayer() -> Json<Value> {
    Command::new("killall")
        .args(&["mplayer"])
        .spawn()
        .expect("Failed to execute killall");

    Json(json!({ "status": "ok", "message": "Killed all mplayer instances" }))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/", get(api::greeting))
                .route("/*any", any(api::fallback))
                .nest(
                    "/v1",
                    Router::new()
                        .route("/killall_mplayer", get(handle_killall_mplayer))
                        .route("/sounds", get(sounds_handler))
                        .route("/play/*name", get(handle_play_sound)),
                ),
        )
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:4242".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod api {
    use super::*;

    pub async fn fallback(_: Uri) -> (StatusCode, Json<Value>) {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "No such API route" })),
        )
    }

    pub async fn greeting(_: Uri) -> (StatusCode, Json<Value>) {
        (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "message": "Welcome to the Realraum Sounds API",
                "server_version": env!("CARGO_PKG_VERSION"),
            })),
        )
    }
}
