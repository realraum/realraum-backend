mod api;
mod compat;
mod data;
mod db;
mod files;
mod playback;

use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use axum::{
    routing::{any, get},
    Router,
};
use lazy_static::lazy_static;
use tower_http::services::{ServeDir, ServeFile};

const BASE_PATH_FALLBACK: &str = "/home/realraum/welcomesounds";

lazy_static! {
    static ref AUDIO_LOCK: Mutex<()> = Mutex::new(());
    pub static ref BASE_PATH: PathBuf = env::var("R3_SOUNDS_BASE_PATH")
        .map_err(|_| ())
        .and_then(|s| Path::new(&s).canonicalize().map_err(|_| ()))
        .unwrap_or_else(|_| Path::new(BASE_PATH_FALLBACK).to_path_buf());
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
        .nest(
            "/compat-sounds",
            Router::new()
                .route("/", get(compat::html_page_handler))
                .nest(
                    "/api-c1",
                    Router::new()
                        .route("/killall_mplayer", get(compat::handle_killall_mplayer))
                        .route("/play/*name", get(compat::handle_play_sound)),
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
