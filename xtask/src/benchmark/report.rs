use std::time::Duration;

use dropsquash_core::EncodeResult;

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkRow {
    pub backend: String,
    pub input: String,
    pub output: String,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub elapsed: Duration,
}

impl BenchmarkRow {
    pub fn from_result(backend: &str, result: EncodeResult, elapsed: Duration) -> Self {
        Self {
            backend: backend.to_string(),
            input: result.input_path.display().to_string(),
            output: result.output_path.display().to_string(),
            original_bytes: result.original_bytes,
            output_bytes: result.output_bytes,
            elapsed,
        }
    }

    fn compression_ratio(&self) -> f64 {
        ratio(self.output_bytes, self.original_bytes)
    }

    fn saved_percent(&self) -> f64 {
        (1.0 - self.compression_ratio()) * 100.0
    }

    fn throughput_mib_s(&self) -> f64 {
        let seconds = self.elapsed.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }
        self.original_bytes as f64 / 1_048_576.0 / seconds
    }
}

pub fn print(rows: &[BenchmarkRow]) {
    print!("{}", csv(rows));
}

pub fn write(path: &std::path::Path, rows: &[BenchmarkRow]) -> Result<(), String> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| std::io::Write::write_all(&mut file, csv(rows).as_bytes()))
        .map_err(|error| format!("failed to write benchmark CSV {}: {error}", path.display()))
}

fn csv(rows: &[BenchmarkRow]) -> String {
    let mut output = String::from(
        "backend,input,output,original_bytes,output_bytes,elapsed_s,compression_ratio,saved_percent,throughput_mib_s\n",
    );
    for row in rows {
        output.push_str(&format!(
            "{},{},{},{},{},{:.3},{:.3},{:.1},{:.3}\n",
            csv_cell(&row.backend),
            csv_cell(&row.input),
            csv_cell(&row.output),
            row.original_bytes,
            row.output_bytes,
            row.elapsed.as_secs_f64(),
            row.compression_ratio(),
            row.saved_percent(),
            row.throughput_mib_s()
        ));
    }
    output
}

fn csv_cell(value: &str) -> String {
    if !value.contains([',', '"', '\n']) {
        return value.to_string();
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        return 0.0;
    }
    numerator as f64 / denominator as f64
}

#[cfg(test)]
mod tests;
