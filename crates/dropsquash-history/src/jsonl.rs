use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{EncodeResult, Result};
use serde::{Deserialize, Serialize};
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionRecord {
    pub recorded_at_unix_seconds: u64,
    pub result: EncodeResult,
}

impl ConversionRecord {
    pub fn new(result: EncodeResult) -> Self {
        Self {
            recorded_at_unix_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or_default(),
            result,
        }
    }
}

pub async fn append_record(path: &Path, record: &ConversionRecord) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    let line = serde_json::to_string(record)?;
    file.write_all(line.as_bytes()).await?;
    file.write_all(b"\n").await?;
    file.flush().await?;
    Ok(())
}

pub async fn read_records(path: &Path) -> Result<Vec<ConversionRecord>> {
    if fs::metadata(path).await.is_err() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(path).await?;
    let mut lines = BufReader::new(file).lines();
    let mut records = Vec::new();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        records.push(serde_json::from_str(&line)?);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dropsquash_core::Profile;
    use tempfile::tempdir;

    use super::*;

    fn result(success: bool) -> EncodeResult {
        EncodeResult {
            input_path: PathBuf::from("input.mov"),
            output_path: PathBuf::from("output.mp4"),
            profile: Profile::Auto,
            original_bytes: 100,
            output_bytes: 20,
            success,
            error_message: None,
        }
    }

    #[tokio::test]
    async fn writes_and_reads_jsonl_records() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("history.jsonl");

        append_record(&path, &ConversionRecord::new(result(true)))
            .await
            .unwrap();
        append_record(&path, &ConversionRecord::new(result(false)))
            .await
            .unwrap();

        let records = read_records(&path).await.unwrap();
        assert_eq!(records.len(), 2);
        assert!(records[0].result.success);
        assert!(!records[1].result.success);
    }

    #[tokio::test]
    async fn missing_history_file_reads_as_empty() {
        let dir = tempdir().expect("temp dir");
        let records = read_records(&dir.path().join("missing.jsonl"))
            .await
            .unwrap();
        assert!(records.is_empty());
    }
}
