use std::path::PathBuf;

mod phases;
mod section;
mod row_notes;
mod sample_hints;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-pending <manual-qa.md> [--section packaged-app|license|benchmark|distribution|local-proof]";

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (path, section) = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let pending = pending_rows(&text);
    if pending.is_empty() {
        println!("manual QA has no pending result rows");
        return Ok(());
    }
    let groups = section::grouped(&pending, section.as_deref());
    for (section, rows) in &groups {
        println!("{section}:");
        if matches!(
            *section,
            "Packaged App" | "License Sandbox" | "Distribution And Signing"
        ) {
            let counts = phases::counts(rows);
            if !counts.is_empty() {
                let summary = counts
                    .iter()
                    .map(|(phase, count)| format!("{phase}={count}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("  phase counts: {summary}");
            }
        }
        let mut current_phase = None;
        for (label, expected) in rows {
            let next_phase = phases::for_label(label);
            if next_phase != current_phase {
                if let Some(phase) = next_phase {
                    println!("  {phase}:");
                }
                current_phase = next_phase;
            }
            println!("- {label}: {expected}");
            if let Some(note) = row_notes::for_label(label) {
                println!("  {note}");
            }
            if let Some(guidance) = sample_hints::guidance_for(label, &text) {
                println!("  {guidance}");
            }
        }
    }
    if includes_packaged_app(&groups) {
        for line in sample_hints::for_manual(&text) {
            println!("{line}");
        }
        if let Some(artifact) = field_value(&text, "App artifact") {
            println!("packaged-app artifact: {artifact}");
            println!("packaged-app open command: open -- '{}'", shell_single_quote(artifact));
        }
    }
    if groups.iter().any(|(name, _)| *name == "License Sandbox") {
        if let Some(path) = field_value(&text, "License cache path") {
            println!("license cache path: {path}");
        }
    }
    if groups.iter().any(|(name, _)| *name == "Distribution And Signing") {
        if let Some(artifact) = field_value(&text, "App artifact") {
            println!("distribution artifact: {artifact}");
        }
    }
    Ok(())
}

pub(crate) fn suggested_section(label: &str) -> &'static str {
    section::filter_for_label(label)
}

fn parse_args(args: Vec<String>) -> Result<(PathBuf, Option<String>), String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok((PathBuf::from(path), None)),
        [path, flag, section] if *flag == "--section" => Ok((PathBuf::from(path), Some(section.clone()))),
        _ => Err(USAGE.to_string()),
    }
}

fn pending_rows(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(pending_row)
        .map(|(label, expected)| (label.to_string(), expected.to_string()))
        .collect()
}

fn pending_row(line: &str) -> Option<(&str, &str)> {
    if !line.starts_with('|') || line.contains("---") {
        return None;
    }
    let cells = line.trim_matches('|').split('|').collect::<Vec<_>>();
    match cells.as_slice() {
        [label, expected, result] if result.trim().is_empty() => Some((label.trim(), expected.trim())),
        [label, expected, _, result] if result.trim().is_empty() => Some((label.trim(), expected.trim())),
        _ => None,
    }
}

fn includes_packaged_app(groups: &[(&str, Vec<(String, String)>)]) -> bool {
    groups.iter().any(|(name, _)| *name == "Packaged App")
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line.trim_matches('|').split('|').map(str::trim).collect::<Vec<_>>();
        match cells.as_slice() {
            [found, value] if *found == label => Some(*value),
            _ => None,
        }
    })
}

fn shell_single_quote(text: &str) -> String {
    text.replace('\'', "'\\''")
}
