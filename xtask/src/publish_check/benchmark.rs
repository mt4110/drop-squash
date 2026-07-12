use std::path::{Path, PathBuf};

pub(super) fn require_notes_csv_matches_manual_qa(
    notes: &str,
    manual_qa_path: &Path,
) -> Result<(), String> {
    let notes_csv = release_notes_csv(notes)?;
    let manual = std::fs::read_to_string(manual_qa_path).map_err(|error| {
        format!(
            "failed to read manual QA {}: {error}",
            manual_qa_path.display()
        )
    })?;
    let manual_csv = manual_qa_csv(&manual)?;
    if notes_csv == manual_csv {
        return Ok(());
    }
    Err("release notes Benchmark sample set CSV path must match manual QA".to_string())
}

fn release_notes_csv(text: &str) -> Result<PathBuf, String> {
    let value = field_value(text, "Benchmark sample set")
        .ok_or_else(|| "release notes Benchmark sample set must be present".to_string())?;
    let path = csv_path(value).ok_or_else(|| {
        "release notes Benchmark sample set must include an absolute .csv path".to_string()
    })?;
    require_outside_repo(&path, "release notes Benchmark sample set")?;
    Ok(path)
}

fn manual_qa_csv(text: &str) -> Result<PathBuf, String> {
    let value = table_result(text, "Benchmark sample set")
        .ok_or_else(|| "manual QA Benchmark sample set must be present".to_string())?;
    let path = csv_path(value).ok_or_else(|| {
        "manual QA Benchmark sample set must include an absolute .csv path".to_string()
    })?;
    require_outside_repo(&path, "manual QA Benchmark sample set")?;
    Ok(path)
}

fn field_value<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    let prefix = format!("- {label}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&prefix).map(str::trim))
}

fn table_result<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        (cells.len() >= 3 && cells.first() == Some(&label)).then_some(cells[2])
    })
}

fn csv_path(value: &str) -> Option<PathBuf> {
    value
        .split_whitespace()
        .map(|token| {
            token.trim_matches(|character: char| {
                matches!(character, ',' | '.' | ';' | ')' | '(' | '`')
            })
        })
        .find(|token| token.starts_with('/') && token.ends_with(".csv"))
        .map(PathBuf::from)
}

fn require_outside_repo(path: &Path, label: &str) -> Result<(), String> {
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    if path.starts_with(cwd) {
        return Err(format!("{label} CSV path must stay outside the repository"));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
