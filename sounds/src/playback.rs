use super::*;

use std::process::Command;

/// Plays a sound from a given path using `mplayer` (not anymore, now using `rodio`).
///
/// Spawns a new child process and returns immediately.  
/// Multiple sounds are prevented by using a global lock.
pub fn play_sound_from_path(filepath: &str) -> bool {
    Command::new("mplayer")
        .args(&[
            "-really-quiet",
            "-nolirc",
            "-ao",
            "alsa",
            &format!("{}/{}", BASE_PATH.display(), filepath),
        ])
        .spawn()
        .expect("Failed to execute mplayer");

    // TODO migrate back to rodio once cancelling sounds is implemented,
    //  and once the 5 second sleep hack has been removed.
    return true;

    // // Get a output stream handle to the default physical sound device
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //
    // let convert_samples = {
    //     // Load a sound from a file, using a path relative to Cargo.toml
    //     let file = BufReader::new(File::open(BASE_PATH.join(filepath)).unwrap());
    //     // Decode that sound file into a source
    //     Decoder::new(file).unwrap().convert_samples()
    // };
    //
    // // HACK Play the sound directly on the device in a new thread.
    // let has_played = std::thread::spawn(move || {
    //     // This lock is used to avoid playing multiple sounds at the same time.
    //     // It can be safely removed, if playing multiple sounds at the same time is desired.
    //     let Ok(_nyaaa) = AUDIO_LOCK.try_lock() else {
    //         // log::warn!("Failed to lock audio lock, another sound is playing");
    //         return false;
    //     };
    //
    //     stream_handle.play_raw(convert_samples).unwrap();
    //
    //     // The sound plays in a separate audio thread,
    //     // so we need to keep the main thread alive while it's playing.
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //
    //     // Do this explicitly for clarity
    //     drop(_nyaaa);
    //     true
    // })
    // .join()
    // .unwrap();
    // // stream_handle.play_raw(convert_samples).unwrap();
    //
    // // The sound plays in a separate audio thread,
    // // so we need to keep the main thread alive while it's playing.
    // // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // has_played
}
