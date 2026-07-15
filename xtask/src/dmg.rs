use std::path::Path;
use std::thread;
use std::time::Duration;

const UDIF_TRAILER_SIZE: usize = 512;
const UDIF_MAGIC: &[u8] = b"koly";
const READ_RETRIES: usize = 5;
const READ_RETRY_DELAY_MS: u64 = 100;

pub(crate) fn read(path: &Path, label: &str) -> Result<Vec<u8>, String> {
    if path.exists() && !path.is_file() {
        return Err(format!("{label} target is not a file: {}", path.display()));
    }
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return Err(format!("{label} target must be a DMG: {}", path.display()));
    }
    let bytes = read_bytes(path)
        .ok_or_else(|| format!("{label} target is not a file: {}", path.display()))?
        .map_err(|error| error.to_string())?;
    if bytes.is_empty() {
        return Err(format!("{label} target is empty: {}", path.display()));
    }
    if !has_udif_trailer(&bytes) {
        return Err(format!(
            "{label} target is not a UDIF DMG: {}",
            path.display()
        ));
    }
    Ok(bytes)
}

fn read_bytes(path: &Path) -> Option<Result<Vec<u8>, std::io::Error>> {
    for attempt in 0..=READ_RETRIES {
        if path.is_file() {
            return Some(std::fs::read(path));
        }
        if attempt < READ_RETRIES {
            thread::sleep(Duration::from_millis(READ_RETRY_DELAY_MS));
        }
    }
    None
}

fn has_udif_trailer(bytes: &[u8]) -> bool {
    if bytes.len() < UDIF_TRAILER_SIZE {
        return false;
    }
    let start = bytes.len() - UDIF_TRAILER_SIZE;
    &bytes[start..start + UDIF_MAGIC.len()] == UDIF_MAGIC
}

#[cfg(test)]
mod tests {
    use super::read_bytes;
    use std::io::ErrorKind;
    use std::time::Duration;

    #[test]
    fn waits_for_recently_created_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("DropSquash.dmg");
        let delayed = path.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(150));
            std::fs::write(delayed, b"later").unwrap();
        });

        let result = read_bytes(&path).unwrap().unwrap();

        assert_eq!(result, b"later");
    }

    #[test]
    fn returns_none_when_file_never_appears() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("missing.dmg");

        assert!(read_bytes(&path).is_none());
    }

    #[test]
    fn preserves_io_errors_for_existing_file() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("folder.dmg");
        std::fs::create_dir(&path).unwrap();

        let error = read_bytes(&path).unwrap().unwrap_err();

        assert_eq!(error.kind(), ErrorKind::IsADirectory);
    }
}
