use std::path::PathBuf;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-pending <manual-qa.md>";
const PACKAGED_APP: &[&str] = &[
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
const LICENSE: &[&str] = &[
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

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let pending = pending_rows(&text);
    if pending.is_empty() {
        println!("manual QA has no pending result rows");
        return Ok(());
    }
    for (section, rows) in grouped(&pending) {
        println!("{section}:");
        for (label, expected) in rows {
            println!("- {label}: {expected}");
        }
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn pending_rows(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(pending_row)
        .map(|cells| (cells[0].trim().to_string(), cells[1].trim().to_string()))
        .collect()
}

fn grouped(rows: &[(String, String)]) -> Vec<(&'static str, Vec<(String, String)>)> {
    let mut packaged = Vec::new();
    let mut license = Vec::new();
    let mut distribution = Vec::new();
    for (label, expected) in rows {
        let row = (label.clone(), expected.clone());
        if PACKAGED_APP.contains(&label.as_str()) {
            packaged.push(row);
        } else if LICENSE.contains(&label.as_str()) {
            license.push(row);
        } else {
            distribution.push(row);
        }
    }
    let mut groups = Vec::new();
    push_group(&mut groups, "Packaged App", packaged);
    push_group(&mut groups, "License Sandbox", license);
    push_group(&mut groups, "Distribution And Signing", distribution);
    groups
}

fn push_group(
    groups: &mut Vec<(&'static str, Vec<(String, String)>)>,
    name: &'static str,
    rows: Vec<(String, String)>,
) {
    if !rows.is_empty() {
        groups.push((name, rows));
    }
}

fn pending_row(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') || line.contains("---") {
        return None;
    }
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    if cells.len() != 4 || !cells[3].trim().is_empty() {
        return None;
    }
    Some(cells)
}
