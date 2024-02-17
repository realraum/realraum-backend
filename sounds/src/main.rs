mod api;
mod data;
mod db;
mod files;
mod playback;

use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    routing::{any, get},
    Json, Router,
};
use lazy_static::lazy_static;
// use rodio::{source::Source, Decoder, OutputStream};
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
    pub static ref BASE_PATH: PathBuf = env::var("R3_SOUNDS_BASE_PATH")
        .map_err(|_| ())
        .and_then(|s| FsPath::new(&s).canonicalize().map_err(|_| ()))
        .unwrap_or_else(|_| FsPath::new(BASE_PATH_FALLBACK).to_path_buf());
    // static ref PIPE_OGG
}

#[tokio::main]
async fn main() -> Result<()> {
    let db_con = db::make_some_db()?;

    for sound in files::index_sounds_from_disk(&BASE_PATH) {
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
                        .route("/killall_mplayer", get(api::handle_killall_mplayer))
                        .route("/sounds", get(api::sounds_handler))
                        .route("/play/*name", get(api::handle_play_sound)),
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
