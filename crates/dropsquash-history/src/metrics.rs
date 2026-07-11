use crate::ConversionRecord;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct HistoryMetrics {
    pub successful_conversion_count: u32,
    pub total_original_bytes: u64,
    pub total_output_bytes: u64,
    pub saved_bytes: u64,
    pub average_reduction_percent: f64,
}

impl HistoryMetrics {
    pub fn from_records(records: &[ConversionRecord]) -> Self {
        let mut metrics = Self::default();

        for record in records
            .iter()
            .filter(|record| record.result.is_successful_conversion())
        {
            metrics.successful_conversion_count += 1;
            metrics.total_original_bytes += record.result.original_bytes;
            metrics.total_output_bytes += record.result.output_bytes;
            metrics.saved_bytes += record.result.saved_bytes();
        }

        if metrics.total_original_bytes > 0 {
            metrics.average_reduction_percent =
                metrics.saved_bytes as f64 / metrics.total_original_bytes as f64 * 100.0;
        }

        metrics
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dropsquash_core::{EncodeResult, Profile};

    use crate::ConversionRecord;

    use super::*;

    fn record(success: bool, original_bytes: u64, output_bytes: u64) -> ConversionRecord {
        ConversionRecord {
            recorded_at_unix_seconds: 1,
            result: EncodeResult {
                input_path: PathBuf::from("input.mov"),
                output_path: PathBuf::from("output.mp4"),
                profile: Profile::Auto,
                original_bytes,
                output_bytes,
                success,
                error_message: None,
            },
        }
    }

    #[test]
    fn aggregates_only_successful_smaller_outputs() {
        let records = vec![
            record(true, 100, 20),
            record(false, 100, 10),
            record(true, 100, 120),
            record(true, 50, 25),
        ];

        let metrics = HistoryMetrics::from_records(&records);

        assert_eq!(metrics.successful_conversion_count, 2);
        assert_eq!(metrics.total_original_bytes, 150);
        assert_eq!(metrics.total_output_bytes, 45);
        assert_eq!(metrics.saved_bytes, 105);
        assert_eq!(metrics.average_reduction_percent, 70.0);
    }
}
