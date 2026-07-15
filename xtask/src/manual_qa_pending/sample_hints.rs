pub(super) fn for_manual(text: &str) -> Vec<String> {
    table_result(text, "Benchmark sample set")
        .and_then(crate::csv_evidence::existing_outside_repo_path)
        .and_then(|csv| sample_hints(&csv).ok())
        .unwrap_or_default()
}

fn table_result<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        match cells.as_slice() {
            [row, _, result] | [row, _, _, result] if *row == label => Some(*result),
            _ => None,
        }
    })
}

fn sample_hints(csv: &std::path::Path) -> Result<Vec<String>, String> {
    let rows = crate::benchmark_csv_check::read_rows(csv)?;
    let samples = rows.iter().skip(1).filter_map(|row| row.get(1)).collect::<Vec<_>>();
    let [small, medium, large] = samples.as_slice() else {
        return Err("benchmark CSV must contain exactly three sample rows".to_string());
    };
    Ok(vec![
        format!("packaged-app small sample: {small}"),
        format!("packaged-app duplicate sample: {small}"),
        format!("packaged-app queue sample set: {small}, {medium}, {large}"),
        format!("packaged-app large sample: {large}"),
    ])
}

#[cfg(test)]
mod tests {
    use super::for_manual;

    #[test]
    fn reports_hints_from_benchmark_sample_set_csv() {
        let directory = tempfile::tempdir().unwrap();
        let csv = directory.path().join("results.csv");
        std::fs::write(
            &csv,
            "backend,input\napple-native,/tmp/short.mov\napple-native,/tmp/medium.mov\napple-native,/tmp/large.mov\n",
        )
        .unwrap();

        let text = format!(
            "| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        );

        let hints = for_manual(&text);

        assert_eq!(hints[0], "packaged-app small sample: /tmp/short.mov");
        assert_eq!(hints[1], "packaged-app duplicate sample: /tmp/short.mov");
        assert_eq!(
            hints[2],
            "packaged-app queue sample set: /tmp/short.mov, /tmp/medium.mov, /tmp/large.mov"
        );
        assert_eq!(hints[3], "packaged-app large sample: /tmp/large.mov");
    }
}
