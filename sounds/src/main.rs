mod db;

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    routing::{any, get},
    Json, Router,
};
use lazy_static::lazy_static;
use rodio::{source::Source, Decoder, OutputStream};
use rusqlite::Connection;
// use rusqlite::NO_PARAMS;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::{ServeDir, ServeFile};

use std::{
    env,
    fs::{self, File},
    io::BufReader,
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};

const BASE_PATH_FALLBACK: &str = "/home/realraum/welcomesounds";

lazy_static! {
    static ref AUDIO_LOCK: Mutex<()> = Mutex::new(());
    static ref BASE_PATH: PathBuf = env::var("R3_SOUNDS_BASE_PATH")
        .map_err(|_| ())
        .and_then(|s| FsPath::new(&s).canonicalize().map_err(|_| ()))
        .unwrap_or_else(|_| FsPath::new(BASE_PATH_FALLBACK).to_path_buf());
    // static ref PIPE_OGG
}

#[derive(Debug, Serialize)]
struct Sound {
    name: String,
    path: String,
    md5sum: [u8; 16],
    id: i64,
    play_count: i64,
    // last_played: Option<DateTime<Utc>>,
}

/// Plays a sound from a given path using `mplayer` (not anymore, now using `rodio`).
///
/// Spawns a new child process and returns immediately.  
/// Multiple sounds are prevented by using a global lock.
pub fn play_sound_from_path(filepath: &str) -> bool {
    // Command::new("mplayer")
    //     .args(&[
    //         "-really-quiet",
    //         "-nolirc",
    //         "-ao",
    //         "alsa",
    //         &format!("{}/{}", BASE_PATH.display(), filepath),
    //     ])
    //     .spawn()
    //     .expect("Failed to execute mplayer");

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let convert_samples = {
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(BASE_PATH.join(filepath)).unwrap());
        // Decode that sound file into a source
        Decoder::new(file).unwrap().convert_samples()
    };

    // HACK Play the sound directly on the device in a new thread.
    let has_played = std::thread::spawn(move || {
        // This lock is used to avoid playing multiple sounds at the same time.
        // It can be safely removed, if playing multiple sounds at the same time is desired.
        let Ok(_nyaaa) = AUDIO_LOCK.try_lock() else {
            // log::warn!("Failed to lock audio lock, another sound is playing");
            return false;
        };

        stream_handle.play_raw(convert_samples).unwrap();

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Do this explicitly for clarity
        drop(_nyaaa);
        true
    })
    .join()
    .unwrap();
    // stream_handle.play_raw(convert_samples).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    has_played
}

/// Lists all sounds in the [`BASE_PATH`] directory, returning a [`Vec`] of [`Sound`] structs.
fn index_sounds_from_disk(base_path: &PathBuf) -> Vec<Sound> {
    let mut sounds = Vec::new();
    println!("Searching for sounds in {}", base_path.display());
    for entry in fs::read_dir(&*base_path).unwrap() {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                let filepath = entry.path();
                let fname = filepath.strip_prefix(&*base_path).unwrap_or(&filepath);
                let fname = fname.to_str().unwrap_or("");
                let Ok(file_contents_bin) = fs::read(&filepath) else {
                    // TODO handle subdirectories
                    println!("Failed to read file {}", filename);
                    continue;
                };
                let md5sum: [u8; 16] = md5::compute(&file_contents_bin).0;
                sounds.push(Sound {
                    name: filename.to_string(),
                    path: fname.to_string(),
                    md5sum,
                    id: 0,
                    play_count: 0,
                });
            }
        }
    }
    sounds
}

/// API endpoint for listing all sounds on `/api/sounds`
async fn sounds_handler(State(db): State<Arc<Mutex<Connection>>>) -> Json<Value> {
    let sounds = db::get_sounds_list(&db.lock().unwrap()).unwrap();
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

async fn handle_play_sound(
    Path(sound_path): Path<String>,
    State(db_con): State<Arc<Mutex<Connection>>>,
) -> Json<Value> {
    let mut base_path = BASE_PATH.clone();
    dbg!(&sound_path);
    base_path.push(&sound_path);
    let filepath = base_path;
    dbg!(&filepath);
    if filepath.exists() {
        dbg!("exists");
        // let _lock = AUDIO_LOCK.lock().unwrap();

        let has_played = play_sound_from_path(&sound_path);
        Json(json!({ "status": "ok", "has_played": has_played }))
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
async fn main() -> Result<()> {
    let db_con = db::make_some_db()?;

    for sound in index_sounds_from_disk(&BASE_PATH) {
        let result = db::insert_sound(&db_con, &sound);
        if result.is_ok() {
            println!("Inserted sound {}", sound.name);
        }
    }

    let db_con = Arc::new(Mutex::new(db_con));

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
        )
        .with_state(db_con);

    // run it with hyper on localhost:3000
    // axum::Server::bind(&"192.168.127.246:80".parse().unwrap())
    let addr: SocketAddr = env::var("R3_SOUNDS_ADDR")
        .map_err(|_| ())
        .and_then(|s| s.parse().map_err(|_| ()))
        .unwrap_or_else(|_| "0.0.0.0:4242".parse().unwrap());

    println!("Starting server on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
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
