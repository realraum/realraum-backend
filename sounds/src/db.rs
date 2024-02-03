// use chrono::{DateTime, TimeZone, Utc};
use rusqlite::Connection;

use anyhow::{Context, Result};

use crate::Sound;

pub fn make_some_db() -> Result<Connection> {
    let db = Connection::open("sounds.db")?;

    // db.execute(
    //     "DROP TABLE IF EXISTS sounds",
    //     // "DROP TABLE IF EXISTS sound_events",
    //     [],
    // )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS sounds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            md5sum BLOB NOT NULL UNIQUE,
            play_count INTEGER DEFAULT 0
        )",
        // last_played DATETIME
        [],
    )?;

    // db.execute(
    //     "CREATE TABLE IF NOT EXISTS sound_events (
    //         id UID PRIMARY KEY,
    //         sound_id UID NOT NULL,
    //         timestamp DATETIME NOT NULL,
    //         FOREIGN KEY (sound_id) REFERENCES sounds(id)
    //     )",
    //     [],
    // )?;

    Ok(db)
}

pub fn increment_play_count(db: &Connection, sound_id: i64) -> Result<()> {
    db.execute(
        "UPDATE sounds SET play_count = play_count + 1 WHERE id = ?",
        &[&sound_id],
    )
    .context("Failed to increment play count")?;

    Ok(())
}

pub fn get_sound_by_name(db: &Connection, name: &str) -> Result<Option<Sound>> {
    let mut stmt = db.prepare("SELECT * FROM sounds WHERE name = ?")?;
    let mut rows = stmt.query(&[&name])?;

    if let Some(row) = rows.next()? {
        // let timestamp: Option<i64> = row.get(5)?;
        // let a = Utc::now();
        // let last_played = timestamp.map(|ts| DateTime::from(ts));
        Ok(Some(Sound {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            md5sum: row.get(3)?,
            play_count: row.get(4)?,
            // last_played,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_sound_by_id(db: &Connection, id: i64) -> Result<Option<Sound>> {
    let mut stmt = db.prepare("SELECT * FROM sounds WHERE id = ?")?;
    let mut rows = stmt.query(&[&id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Sound {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            md5sum: row.get(3)?,
            play_count: row.get(4)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_sounds_list(db: &Connection) -> Result<Vec<Sound>> {
    let mut stmt = db
        .prepare("SELECT * FROM sounds")
        .context("Failed to prepare get_sounds_list")?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Sound {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                md5sum: row.get(3)?,
                play_count: row.get(4)?,
            })
        })
        .context("Failed to query_map get_sounds_list")?;

    let mut sounds = Vec::new();
    for sound in rows {
        sounds.push(sound?);
    }

    Ok(sounds)
}

pub fn insert_sound(db: &Connection, sound: &Sound) -> Result<()> {
    db.execute(
        "INSERT INTO sounds (name, path, md5sum, play_count) VALUES (?, ?, ?, ?)",
        (&sound.name, &sound.path, &sound.md5sum, &sound.play_count),
    )
    .context("Failed to insert sound")?;

    Ok(())
}
