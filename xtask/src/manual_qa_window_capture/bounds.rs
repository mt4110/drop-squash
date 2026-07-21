pub(super) fn parse(bounds: &str) -> Result<(), String> {
    let values = bounds
        .split(',')
        .map(str::trim)
        .map(|value| value.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to parse window bounds: {error}"))?;
    if values.len() != 4 {
        return Err(format!("expected 4 window bounds values, got {bounds}"));
    }
    if values[2] <= 0 || values[3] <= 0 {
        return Err(format!("window size must be positive: {bounds}"));
    }
    Ok(())
}
