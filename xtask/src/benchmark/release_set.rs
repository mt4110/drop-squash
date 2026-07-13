use super::report::BenchmarkRow;

pub(super) fn require_timing_evidence(rows: &[BenchmarkRow]) -> Result<(), String> {
    for row in rows {
        if row.duration.is_none() || row.speed_ratio().is_none() {
            return Err(format!(
                "benchmark --release-set requires duration and speed ratio evidence: {}",
                row.input
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
