use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaInfo {
    pub path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<Duration>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub size_bytes: u64,
}
