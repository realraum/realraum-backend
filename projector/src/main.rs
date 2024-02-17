use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

use anyhow::Result;
use axum::{
    http::HeaderValue,
    routing::{any, get},
    Json, Router,
};
use hyper::{Method, StatusCode, Uri};
use protocol::{
    commands::{input, menu, picture, power, volume},
    Command,
};
use serde_json::{json, Value};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

pub mod protocol;

// This is hard-coded for now
// TODO make this configurable to account for DHCP
// Or just use a fixed IP address in the projector like we did
const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 33, 41);

// TODO make this configurable
const PORT: u16 = 41794;

#[tokio::main]
async fn main() -> Result<()> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .layer(cors)
        .nest(
            "/api",
            Router::new()
                .route("/", get(api::greeting))
                .route("/*any", any(api::fallback))
                .nest(
                    "/v1",
                    Router::new()
                        .nest(
                            "/input",
                            Router::new()
                                .route("/vga_a", get(|| handle_command(input::VGA_A)))
                                .route("/vga_b", get(|| handle_command(input::VGA_B)))
                                .route("/composite", get(|| handle_command(input::COMPOSITE)))
                                .route("/s_video", get(|| handle_command(input::S_VIDEO)))
                                .route("/hdmi", get(|| handle_command(input::HDMI)))
                                .route("/wireless", get(|| handle_command(input::WIRELESS)))
                                .route("/usb_display", get(|| handle_command(input::USB_DISPLAY)))
                                .route("/usb_viewer", get(|| handle_command(input::USB_VIEWER))),
                        )
                        .nest(
                            "/volume",
                            Router::new()
                                .route("/up", get(|| handle_command(volume::UP)))
                                .route("/down", get(|| handle_command(volume::DOWN)))
                                .route("/mute", get(|| handle_command(volume::MUTE)))
                                .route("/un_mute", get(|| handle_command(volume::UN_MUTE))),
                        )
                        .nest(
                            "/power",
                            Router::new()
                                .route("/on", get(|| handle_command(power::ON)))
                                .route("/off", get(|| handle_command(power::OFF))),
                        )
                        .nest(
                            "/menu",
                            Router::new()
                                .route("/menu_button", get(|| handle_command(menu::MENU_BUTTON)))
                                .route("/up", get(|| handle_command(menu::UP)))
                                .route("/down", get(|| handle_command(menu::DOWN)))
                                .route(
                                    "/left",
                                    get(|| handle_command(menu::LEFT)),
                                    // .route(
                                    //     "/left",
                                    //     get(|State(state): State<Arc<Mutex<TcpStream>>>| {
                                    //         handle_command(menu::LEFT, state)
                                    //     }),
                                )
                                .route("/right", get(|| handle_command(menu::RIGHT)))
                                .route("/ok", get(|| handle_command(menu::OK))),
                        )
                        .nest(
                            "/picture",
                            Router::new()
                                .route("/blank", get(|| handle_command(picture::BLANK)))
                                .route("/un_blank", get(|| handle_command(picture::UN_BLANK)))
                                .route("/freeze", get(|| handle_command(picture::FREEZE)))
                                .route("/un_freeze", get(|| handle_command(picture::UN_FREEZE)))
                                .route("/contrast_up", get(|| handle_command(picture::CONTRAST_UP)))
                                .route(
                                    "/contrast_down",
                                    get(|| handle_command(picture::CONTRAST_DOWN)),
                                )
                                .route(
                                    "/brightness_up",
                                    get(|| handle_command(picture::BRIGHTNESS_UP)),
                                )
                                .route(
                                    "/brightness_down",
                                    get(|| handle_command(picture::BRIGHTNESS_DOWN)),
                                ),
                        ),
                ),
        )
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        )
        // .with_state(connection)
        ;

    let addr: SocketAddr = env::var("R3_PROJECTOR_ADDR")
        .map_err(|_| ())
        .and_then(|s| s.parse().map_err(|_| ()))
        .unwrap_or_else(|_| "0.0.0.0:4201".parse().unwrap());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// async fn handle_command(command: Command, connection: Arc<Mutex<TcpStream>>) -> Json<Value> {
//     connection
//         .lock()
//         .unwrap()
//         .write_all(dbg!(&commands::power::ON))
//         .await
//         .unwrap();
//     // stream.write_all(&commands::power::OFF).await.unwrap();

//     Json(json!({ "status": "ok", "message": "Killed all mplayer instances" }))
// }

async fn handle_command(command: Command) -> Json<Value> {
    let mut connection = TcpStream::connect((IP_ADDRESS, PORT)).await.unwrap();

    connection.write_all(&command).await.unwrap();
    // stream.write_all(&commands::power::OFF).await.unwrap();

    Json(json!({
        "status": "ok",
        "message": "Command sent successfully",
        "command": &command
    }))
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
                "message": "Welcome to the Realraum Projector-Remote API",
                "server_version": env!("CARGO_PKG_VERSION"),
            })),
        )
    }
}
