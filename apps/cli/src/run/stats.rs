use std::path::PathBuf;

use dropsquash_core::default_history_path;
use dropsquash_history::{read_records, HistoryMetrics};

use super::license;

pub async fn run(history: Option<PathBuf>) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    let records = read_records(&history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    let state = license::gate()?.state_for_metrics(metrics);
    println!(
        "successful conversions: {}",
        metrics.successful_conversion_count
    );
    println!("total original bytes: {}", metrics.total_original_bytes);
    println!("total output bytes: {}", metrics.total_output_bytes);
    println!("saved bytes: {}", metrics.saved_bytes);
    println!(
        "average reduction: {:.2}%",
        metrics.average_reduction_percent
    );
    println!("license state: {state:?}");
    Ok(())
}
