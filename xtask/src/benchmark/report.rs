use std::time::Duration;

use dropsquash_core::EncodeResult;

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkRow {
    pub input: String,
    pub output: String,
    pub original_bytes: u64,
    pub output_bytes: u64,
    pub elapsed: Duration,
}

impl BenchmarkRow {
    pub fn from_result(result: EncodeResult, elapsed: Duration) -> Self {
        Self {
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

    fn throughput_mib_s(&self) -> f64 {
        let seconds = self.elapsed.as_secs_f64();
        if seconds == 0.0 {
            return 0.0;
        }
        self.original_bytes as f64 / 1_048_576.0 / seconds
    }
}

pub fn print(rows: &[BenchmarkRow]) {
    println!(
        "input,output,original_bytes,output_bytes,elapsed_s,compression_ratio,throughput_mib_s"
    );
    for row in rows {
        println!(
            "{},{},{},{},{:.3},{:.3},{:.3}",
            row.input,
            row.output,
            row.original_bytes,
            row.output_bytes,
            row.elapsed.as_secs_f64(),
            row.compression_ratio(),
            row.throughput_mib_s()
        );
    }
}

fn ratio(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        return 0.0;
    }
    numerator as f64 / denominator as f64
}
