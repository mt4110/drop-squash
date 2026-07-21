use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub(super) struct RecordingPaths {
    pub(super) partial: PathBuf,
    pub(super) final_path: PathBuf,
}

pub(super) fn recording_paths(output_dir: PathBuf) -> Result<RecordingPaths, String> {
    std::fs::create_dir_all(&output_dir).map_err(|error| error.to_string())?;
    remove_stale_partials(&output_dir)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_millis();
    let stem = format!("secure-share-recording-{stamp}-{}", std::process::id());
    Ok(RecordingPaths {
        partial: output_dir.join(format!(".{stem}.partial.mp4")),
        final_path: output_dir.join(format!("{stem}.mp4")),
    })
}

fn remove_stale_partials(output_dir: &Path) -> Result<(), String> {
    for entry in std::fs::read_dir(output_dir).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_file() && is_stale_partial(&path) {
            std::fs::remove_file(path).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn is_stale_partial(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    name.starts_with(".secure-share-recording-") && name.ends_with(".partial.mp4")
        || name.starts_with("secure-share-recording-") && name.ends_with(".mask-plan.json.partial")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_only_unpublished_secure_share_artifacts() {
        let directory = tempfile::tempdir().unwrap();
        let stale = directory
            .path()
            .join(".secure-share-recording-1-2.partial.mp4");
        let receipt = directory
            .path()
            .join("secure-share-recording-1-2.mask-plan.json.partial");
        let keep = directory.path().join("recording.partial.mp4");
        std::fs::write(&stale, b"partial").unwrap();
        std::fs::write(&receipt, b"receipt").unwrap();
        std::fs::write(&keep, b"keep").unwrap();

        let _ = recording_paths(directory.path().to_path_buf()).unwrap();

        assert!(!stale.exists());
        assert!(!receipt.exists());
        assert!(keep.exists());
    }
}
