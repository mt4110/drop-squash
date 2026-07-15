pub(super) const PACKAGED_APP: &[&str] = &[
    "Disk image launch notice",
    "Choose recording conversion",
    "Drag-and-drop conversion",
    "Privacy receipt sidecar",
    "Reveal privacy receipt",
    "Duplicate output naming",
    "Cancellation",
    "Multi-file queue",
    "Queued job cancellation",
    "Batch summary",
    "Ask source policy",
    "Trash source policy",
    "Failed conversion",
    "Larger output",
    "Reveal output",
];
pub(super) const LICENSE: &[&str] = &[
    "Sandbox product setup",
    "Sandbox purchase",
    "Empty key activation",
    "Invalid key activation",
    "Valid sandbox activation",
    "License network failure",
    "Expired license refresh",
    "Forget license on this Mac",
    "`cargo run -p dropsquash -- license status`",
];
pub(super) const BENCHMARK: &[&str] = &[
    "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`",
    "Benchmark sample set",
    "Benchmark regression threshold",
];

pub(crate) fn filter_for_label(label: &str) -> &'static str {
    if PACKAGED_APP.contains(&label) {
        "packaged-app"
    } else if LICENSE.contains(&label) {
        "license"
    } else if BENCHMARK.contains(&label) {
        "benchmark"
    } else {
        "distribution"
    }
}

pub(super) fn grouped(
    rows: &[(String, String)],
    filter: Option<&str>,
) -> Vec<(&'static str, Vec<(String, String)>)> {
    let mut packaged = Vec::new();
    let mut license = Vec::new();
    let mut benchmark = Vec::new();
    let mut distribution = Vec::new();
    for (label, expected) in rows {
        let row = (label.clone(), expected.clone());
        if PACKAGED_APP.contains(&label.as_str()) {
            packaged.push(row);
        } else if LICENSE.contains(&label.as_str()) {
            license.push(row);
        } else if BENCHMARK.contains(&label.as_str()) {
            benchmark.push(row);
        } else {
            distribution.push(row);
        }
    }
    let mut groups = Vec::new();
    push_group(&mut groups, "Packaged App", packaged, filter);
    push_group(&mut groups, "License Sandbox", license, filter);
    push_group(&mut groups, "Benchmark Evidence", benchmark, filter);
    push_group(
        &mut groups,
        "Distribution And Signing",
        distribution,
        filter,
    );
    groups
}

fn push_group(
    groups: &mut Vec<(&'static str, Vec<(String, String)>)>,
    name: &'static str,
    rows: Vec<(String, String)>,
    filter: Option<&str>,
) {
    let include = filter.is_none_or(|value| matches_filter(name, value));
    if include && !rows.is_empty() {
        groups.push((name, rows));
    }
}

fn matches_filter(name: &str, filter: &str) -> bool {
    let normalized = filter.to_ascii_lowercase();
    matches!(
        (name, normalized.as_str()),
        ("Packaged App", "packaged-app" | "packaged" | "app" | "local-proof")
            | ("License Sandbox", "license-sandbox" | "license")
            | ("Benchmark Evidence", "benchmark" | "local-proof")
            | ("Distribution And Signing", "distribution" | "signing")
    )
}
