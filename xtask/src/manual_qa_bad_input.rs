use std::path::PathBuf;

const DEFAULT_PATH: &str = "/tmp/dropsquash-manual-qa-invalid.mp4";
const BYTES: &[u8] = b"DropSquash manual QA invalid mp4 placeholder\n";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_path(args)?;
    write_invalid_mp4(&path)?;
    println!("manual QA invalid input: {}", path.display());
    Ok(())
}

fn parse_path(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [] => Ok(PathBuf::from(DEFAULT_PATH)),
        [path] => Ok(PathBuf::from(path)),
        _ => Err(
            "usage: cargo run -p xtask -- manual-qa-bad-input [output.mp4]".to_string(),
        ),
    }
}

fn write_invalid_mp4(path: &PathBuf) -> Result<(), String> {
    if path.extension().and_then(|value| value.to_str()) != Some("mp4") {
        return Err("manual QA bad input must use an .mp4 path".to_string());
    }
    std::fs::write(path, BYTES)
        .map_err(|error| format!("failed to write invalid manual QA input: {error}"))
}

#[cfg(test)]
mod tests {
    use super::{parse_path, run};

    #[test]
    fn defaults_to_tmp_mp4_path() {
        assert_eq!(
            parse_path(Vec::new()).unwrap(),
            std::path::PathBuf::from("/tmp/dropsquash-manual-qa-invalid.mp4")
        );
    }

    #[test]
    fn rejects_non_mp4_output() {
        let error = run(vec!["/tmp/not-video.txt".to_string()]).unwrap_err();
        assert!(error.contains(".mp4"));
    }

    #[test]
    fn writes_invalid_mp4_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("invalid.mp4");

        run(vec![path.display().to_string()]).unwrap();

        let bytes = std::fs::read(path).unwrap();
        assert!(bytes.starts_with(b"DropSquash manual QA invalid mp4"));
    }
}
