const HEADER: &[&str] = &[
    "backend",
    "input",
    "output",
    "original_bytes",
    "output_bytes",
    "duration_s",
    "elapsed_s",
    "compression_ratio",
    "saved_percent",
    "throughput_mib_s",
    "speed_ratio",
];

pub(super) fn validate(rows: &[Vec<String>]) -> Result<(), String> {
    let Some(header) = rows.first() else {
        return Err("benchmark CSV is empty".into());
    };
    if header != HEADER {
        return Err("benchmark CSV header does not match xtask benchmark output".into());
    }
    let body = &rows[1..];
    if body.len() < 3 {
        return Err("benchmark release-set CSV requires at least three sample rows".into());
    }
    validate_unique_paths(body)?;
    for (index, row) in body.iter().enumerate() {
        validate_row(index + 2, row)?;
    }
    Ok(())
}

fn validate_unique_paths(rows: &[Vec<String>]) -> Result<(), String> {
    let mut inputs = std::collections::HashSet::new();
    let mut outputs = std::collections::HashSet::new();
    for (index, row) in rows.iter().enumerate() {
        let line = index + 2;
        let Some(input) = row.get(1) else {
            return Ok(());
        };
        let Some(output) = row.get(2) else {
            return Ok(());
        };
        if !inputs.insert(input) {
            return Err(format!("benchmark CSV line {line} duplicates input path"));
        }
        if !outputs.insert(output) {
            return Err(format!("benchmark CSV line {line} duplicates output path"));
        }
    }
    Ok(())
}

fn validate_row(line: usize, row: &[String]) -> Result<(), String> {
    if row.len() != HEADER.len() {
        return Err(format!("benchmark CSV line {line} has wrong column count"));
    }
    require_text(line, row, 0, "backend")?;
    require_text(line, row, 1, "input")?;
    require_text(line, row, 2, "output")?;
    let original = require_u64(line, row, 3, "original_bytes")?;
    let output = require_u64(line, row, 4, "output_bytes")?;
    if original == 0 || output == 0 {
        return Err(format!("benchmark CSV line {line} has zero byte count"));
    }
    if output >= original {
        return Err(format!("benchmark CSV line {line} output is not smaller"));
    }
    require_f64_range(line, row, 5, "duration_s", 0.0, f64::INFINITY)?;
    require_f64_range(line, row, 6, "elapsed_s", 0.0, f64::INFINITY)?;
    require_f64_range(line, row, 7, "compression_ratio", 0.0, 1.0)?;
    require_f64_range(line, row, 8, "saved_percent", 0.0, 100.0)?;
    require_f64_range(line, row, 9, "throughput_mib_s", 0.0, f64::INFINITY)?;
    require_f64_range(line, row, 10, "speed_ratio", 0.0, f64::INFINITY)?;
    Ok(())
}

fn require_text(line: usize, row: &[String], index: usize, label: &str) -> Result<(), String> {
    if row[index].trim().is_empty() {
        return Err(format!("benchmark CSV line {line} missing {label}"));
    }
    Ok(())
}

fn require_u64(line: usize, row: &[String], index: usize, label: &str) -> Result<u64, String> {
    row[index]
        .parse()
        .map_err(|_| format!("benchmark CSV line {line} has invalid {label}"))
}

fn require_f64(line: usize, row: &[String], index: usize, label: &str) -> Result<f64, String> {
    let value = row[index].trim();
    if value.is_empty() {
        return Err(format!("benchmark CSV line {line} missing {label}"));
    }
    value
        .parse()
        .map_err(|_| format!("benchmark CSV line {line} has invalid {label}"))
}

fn require_f64_range(
    line: usize,
    row: &[String],
    index: usize,
    label: &str,
    min: f64,
    max: f64,
) -> Result<(), String> {
    let value = require_f64(line, row, index, label)?;
    if value <= min {
        return Err(format!(
            "benchmark CSV line {line} has non-positive {label}"
        ));
    }
    if value >= max {
        return Err(format!(
            "benchmark CSV line {line} has out-of-range {label}"
        ));
    }
    Ok(())
}
