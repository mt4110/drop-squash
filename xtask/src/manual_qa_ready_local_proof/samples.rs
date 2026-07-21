use std::path::Path;

pub(super) fn sample_hints(csv: &Path) -> Result<Vec<String>, String> {
    let rows = crate::benchmark_csv_check::read_rows(csv)?;
    let samples = rows
        .iter()
        .skip(1)
        .filter_map(|row| row.get(1))
        .collect::<Vec<_>>();
    let [small, medium, large] = samples.as_slice() else {
        return Err("benchmark CSV must contain exactly three sample rows".to_string());
    };
    Ok(vec![
        format!("packaged-app small sample: {small}"),
        format!("packaged-app duplicate sample: {small}"),
        format!("packaged-app queue sample set: {small}, {medium}, {large}"),
        format!("packaged-app large sample: {large}"),
        format!(
            "packaged-app not-smaller sample: {}",
            rows[1]
                .get(2)
                .ok_or_else(|| "benchmark CSV row is missing output column: small".to_string())?
        ),
    ])
}
