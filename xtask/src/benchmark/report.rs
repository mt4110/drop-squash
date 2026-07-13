use std::time::Duration;

use dropsquash_core::EncodeResult;

mod csv;

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkRow {
    pub backend: String,
    pub input: String,
    pub output: String,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub elapsed: Duration,
    pub duration: Option<Duration>,
}

impl BenchmarkRow {
    pub fn from_result(
        backend: &str,
        result: EncodeResult,
        elapsed: Duration,
        duration: Option<Duration>,
    ) -> Self {
        Self {
            backend: backend.to_string(),
            input: result.input_path.display().to_string(),
            output: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            elapsed,
            duration,
        }
    }

    pub(super) fn compression_ratio(&self) -> f64 {
        ratio(self.output_bytes, self.original_bytes)
    }

    pub(super) fn saved_percent(&self) -> f64 {
        (1.0 - self.compression_ratio()) * 100.0
    }

    pub(super) fn throughput_mib_s(&self) -> f64 {
        let seconds = self.elapsed.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }
        self.original_bytes as f64 / 1_048_576.0 / seconds
    }

    pub(super) fn speed_ratio(&self) -> Option<f64> {
        let elapsed = self.elapsed.as_secs_f64();
        if elapsed == 0.0 {
            return None;
        }
        self.duration
            .map(|duration| duration.as_secs_f64() / elapsed)
    }
}

pub fn print(rows: &[BenchmarkRow]) {
    print!("{}", csv::from_rows(rows));
}

pub fn write(path: &std::path::Path, rows: &[BenchmarkRow]) -> Result<(), String> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, csv::from_rows(rows).as_bytes()))
        .map_err(|error| format!("failed to write benchmark CSV {}: {error}", path.display()))
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        return 0.0;
    }
    numerator as f64 / denominator as f64
}

#[cfg(test)]
mod tests;
