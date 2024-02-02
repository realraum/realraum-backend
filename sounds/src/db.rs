use rusqlite::Connection;

use anyhow::Result;

use crate::Sound;

pub fn make_some_db() -> Result<Connection> {
    let db = Connection::open("sounds.db")?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS sounds (
            id UID PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            md5sum BLOB NOT NULL
        )",
        [],
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS sound_events (
            id UID PRIMARY KEY,
            sound_id UID NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY (sound_id) REFERENCES sounds(id)
        )",
        [],
    )?;

    Ok(db)
}

pub fn increment_play_count(db: &Connection, sound_id: i64) -> Result<()> {
    db.execute(
        "INSERT INTO sound_events (sound_id, timestamp) VALUES (?, datetime('now'))",
        (&sound_id,),
    )?;

    Ok(())
}

pub fn get_sound_id_by_name(db: &Connection, name: &str) -> Result<Option<Sound>> {
    let mut stmt = db.prepare("SELECT * FROM sounds WHERE name = ?")?;
    let mut rows = stmt.query(&[&name])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Sound {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            md5sum: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}
