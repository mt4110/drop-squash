use std::collections::BTreeMap;
use std::path::Path;

pub(super) fn result(current: &Path, baseline: Option<&Path>) -> Result<String, String> {
    if baseline.is_none() {
        return Ok(format!(
            "first release candidate sample set establishes the same-machine release candidate baseline at {}; 20% regression comparison starts with the next release candidate sample set",
            current.display()
        ));
    }
    let baseline = baseline.expect("checked above");
    let current_rows = samples(current)?;
    let baseline_rows = samples(baseline)?;
    if current_rows.len() != baseline_rows.len() {
        return Err("benchmark CSV sample counts do not match".to_string());
    }
    let mut exceeding = Vec::new();
    for (name, current_value) in &current_rows {
        let baseline_value = baseline_rows
            .get(name)
            .ok_or_else(|| format!("baseline CSV is missing sample: {name}"))?;
        let regression = regression_percent(*baseline_value, *current_value)?;
        if regression > 20.0 {
            exceeding.push(format!("{name} {:.1}%", regression));
        }
    }
    if exceeding.len() >= 2 {
        return Err(format!(
            "benchmark threshold failed: {} samples exceeded 20% regression against the same-machine release candidate baseline {}: {}",
            exceeding.len(),
            baseline.display(),
            exceeding.join(", ")
        ));
    }
    if let Some(sample) = exceeding.first() {
        return Ok(format!(
            "1 sample exceeded 20% regression, below the two-sample release threshold, against the same-machine release candidate baseline {}: {}",
            baseline.display(),
            sample
        ));
    }
    Ok(format!(
        "no sample exceeded 20% regression against the same-machine release candidate baseline {}",
        baseline.display()
    ))
}

fn samples(path: &Path) -> Result<BTreeMap<String, f64>, String> {
    let rows = crate::benchmark_csv_check::read_rows(path)?;
    let sample_rows = rows
        .get(1..)
        .ok_or_else(|| "benchmark CSV is missing sample rows".to_string())?;
    if sample_rows.len() != 3 {
        return Err("benchmark CSV must contain exactly three sample rows".to_string());
    }
    let mut samples = BTreeMap::new();
    for row in sample_rows {
        samples.insert(sample_name(row, 1)?.to_string(), number(row, 9)?);
    }
    Ok(samples)
}

fn regression_percent(baseline: f64, current: f64) -> Result<f64, String> {
    if baseline <= 0.0 {
        return Err("baseline throughput must be greater than zero".to_string());
    }
    Ok(((baseline - current) / baseline).max(0.0) * 100.0)
}

fn sample_name(row: &[String], index: usize) -> Result<&str, String> {
    Ok(cell(row, index)?
        .rsplit('/')
        .next()
        .unwrap_or(cell(row, index)?))
}

fn number(row: &[String], index: usize) -> Result<f64, String> {
    cell(row, index)?
        .parse::<f64>()
        .map_err(|_| format!("benchmark CSV has invalid number in column {index}"))
}

fn cell(row: &[String], index: usize) -> Result<&str, String> {
    row.get(index)
        .map(|value| value.as_str())
        .ok_or_else(|| format!("benchmark CSV is missing column {index}"))
}
