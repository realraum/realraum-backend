use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    Json,
};
use hyper::{StatusCode, Uri};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{db, playback::play_sound_from_path, BASE_PATH};

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

/// API endpoint for listing all sounds on `/api/sounds`
pub async fn sounds_handler(State(db): State<Arc<Mutex<Connection>>>) -> Json<Value> {
    let sounds = db::get_sounds_list(&db.lock().unwrap()).unwrap();
    let response = json!(sounds);
    Json(response)
}

/*
pub async fn play_sound_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let filename = req.uri().path().trim_start_matches("/play/");
    let filepath = format!("{}/{}", BASE_PATH, filename);
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
pub struct PlaySoundPayload {
    name: String,
}

pub async fn handle_play_sound(
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
        if has_played {
            let db = db_con.lock().unwrap();
            let sound = db::get_sound_by_name(&db, &sound_path).unwrap().unwrap();
            db::increment_play_count(&db, sound.id).unwrap();
            Json(json!({ "status": "ok", "has_played": has_played }))
        } else {
            Json(json!({
                "status": "error",
                "message": "Failed to play sound",
                "has_played": has_played
            }))
        }
    } else {
        dbg!("not exists");
        Json(json!({ "status": "error", "message": "File not found" }))
    }
}

pub async fn handle_killall_mplayer() -> Json<Value> {
    Command::new("killall")
        .args(&["mplayer"])
        .spawn()
        .expect("Failed to execute killall");

    Json(json!({ "status": "ok", "message": "Killed all mplayer instances" }))
}
