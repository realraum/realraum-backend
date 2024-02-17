use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct Sound {
    pub(crate) name: String,
    pub(crate) path: String,
    #[serde(skip)]
    pub(crate) md5sum: [u8; 16],
    pub(crate) id: i64,
    pub(crate) play_count: i64,
    // last_played: Option<DateTime<Utc>>,
}
