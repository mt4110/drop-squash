use std::path::Path;

use super::{
    complete, distribution, fields, gates, license, packaged_app, pending_rows, phases, row_notes,
    sample_hints, section, USAGE,
};

pub(super) fn pending(args: Vec<String>) -> Result<(), String> {
    let (path, selected) = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    let pending = pending_rows::pending_rows(&text);
    let groups = section::grouped(&pending, selected.as_deref());
    if groups.is_empty() {
        complete::print(selected.as_deref(), &text);
        return Ok(());
    }
    print_groups(&groups, &text);
    print_extras(&groups, &text);
    Ok(())
}

pub(super) fn parse_args(
    args: Vec<String>,
) -> Result<(std::path::PathBuf, Option<String>), String> {
    match args.as_slice() {
        [path] if path != "--help" && path != "-h" => Ok((path.into(), None)),
        [path, flag, section] if *flag == "--section" => Ok((path.into(), Some(section.clone()))),
        _ => Err(USAGE.to_string()),
    }
}

fn print_groups(groups: &[(&'static str, Vec<(String, String)>)], text: &str) {
    for (name, rows) in groups {
        println!("{name}:");
        print_phase_count(name, rows);
        print_rows(rows, text);
    }
    for line in gates::for_groups(groups) {
        println!("{line}");
    }
}

fn print_phase_count(name: &str, rows: &[(String, String)]) {
    if !matches!(
        name,
        "Packaged App" | "License Sandbox" | "Distribution And Signing"
    ) {
        return;
    }
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

fn print_rows(rows: &[(String, String)], text: &str) {
    let mut current = None;
    for (label, expected) in rows {
        let next = phases::for_label(label);
        if next != current {
            if let Some(phase) = next {
                println!("  {phase}:");
            }
            current = next;
        }
        println!("- {label}: {expected}");
        if let Some(note) = row_notes::for_label(label) {
            println!("  {note}");
        }
        if let Some(hint) = sample_hints::guidance_for(label, text) {
            println!("  {hint}");
        }
    }
}

fn print_extras(groups: &[(&'static str, Vec<(String, String)>)], text: &str) {
    if pending_rows::includes_packaged_app(groups) {
        print_lines(sample_hints::for_manual(text));
        print_lines(packaged_app::extra_lines(text));
    }
    if groups.iter().any(|(name, _)| *name == "License Sandbox") {
        print_lines(license::extra_lines(text, groups));
    }
    if groups.iter().any(|(name, _)| *name == "Benchmark Evidence") {
        print_lines(sample_hints::for_benchmark(text));
    }
    if groups
        .iter()
        .any(|(name, _)| *name == "Distribution And Signing")
    {
        print_distribution(groups, text);
    }
}

fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("{line}");
    }
}

fn print_distribution(groups: &[(&'static str, Vec<(String, String)>)], text: &str) {
    print_lines(distribution::extra_lines(text, groups));
    if let Some(artifact) = fields::field_value(text, "App artifact") {
        println!("distribution artifact: {artifact}");
    }
    println!("distribution cask path: packaging/homebrew/Casks/dropsquash.rb");
    if let Some(path) = fields::field_value(text, "Output folder") {
        println!(
            "distribution checksum path: {}",
            Path::new(path).join("SHA256SUMS").display()
        );
    }
}
