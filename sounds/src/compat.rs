use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
};
use rusqlite::Connection;

use crate::{db, playback::play_sound_from_path, BASE_PATH};

/// API endpoint for listing all sounds on `/api/sounds`
pub async fn html_page_handler(State(db): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let sounds = db::get_sounds_list(&db.lock().unwrap()).unwrap();

    let mut html = String::from(
        "<html><head><title>realraum Sounds</title></head><body><h1>realraum Sounds</h1>",
    );

    html.push_str(&format!(
        "<p>Version {} &nbsp; - &nbsp;",
        env!("CARGO_PKG_VERSION")
    ));

    html.push_str(
        "<a href=\"/compat-sounds/api-c1/killall_mplayer\">Kill all mplayer instances</a></p>",
    );

    let mut sounds = sounds;
    sounds.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    sounds.sort_by(|a, b| b.play_count.cmp(&a.play_count));

    for sound in sounds {
        html.push_str(&format!(
            "<a href=\"/compat-sounds/api-c1/play/{}\">{}</a> (played {} times)<br>",
            sound.name, sound.name, sound.play_count
        ));
    }

    html.push_str("</body></html>");

    Html(html)
}

pub async fn handle_play_sound(
    Path(sound_path): Path<String>,
    State(db_con): State<Arc<Mutex<Connection>>>,
) -> impl IntoResponse {
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
        }
    }

    // We don't show errors to the user in the compat html page
    Redirect::temporary("/compat-sounds")
}

pub async fn handle_killall_mplayer() -> impl IntoResponse {
    Command::new("killall")
        .args(&["mplayer"])
        .spawn()
        .expect("Failed to execute killall");

    // We don't show errors to the user in the compat html page
    Redirect::temporary("/compat-sounds")
}
