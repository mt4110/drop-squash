use std::path::Path;

use dropsquash_core::{AppError, MediaInfo, Result};

use crate::inspect_mp4;

pub fn probe(path: &Path) -> Result<MediaInfo> {
    let metadata = std::fs::metadata(path).map_err(|error| match error.kind() {
        std::io::ErrorKind::NotFound => AppError::FileNotFound(path.to_path_buf()),
        _ => AppError::Io(error),
    })?;

    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if !matches!(extension.as_str(), "mov" | "mp4" | "m4v" | "mkv") {
        return Err(AppError::UnsupportedMedia(extension));
    }

    let duration = if matches!(extension.as_str(), "mov" | "mp4" | "m4v") {
        inspect_mp4(path).duration
    } else {
        None
    };

    Ok(MediaInfo {
        path: path.to_path_buf(),
        width: None,
        height: None,
        duration,
        video_codec: None,
        audio_codec: None,
        size_bytes: metadata.len(),
    })
}
