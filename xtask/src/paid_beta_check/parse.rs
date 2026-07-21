use std::collections::BTreeMap;
use std::path::PathBuf;

const USAGE: &str =
    "usage: cargo run -p xtask -- paid-beta-check [release-blockers.md] [paid-beta-readiness.md] [manual-qa.md]";

pub(super) fn parse_args(args: Vec<String>) -> Result<(PathBuf, PathBuf, PathBuf), String> {
    match args.as_slice() {
        [] => Ok(default_paths()),
        [blockers] => Ok((
            PathBuf::from(blockers),
            default_paths().1,
            default_paths().2,
        )),
        [blockers, readiness] => Ok((
            PathBuf::from(blockers),
            PathBuf::from(readiness),
            default_paths().2,
        )),
        [blockers, readiness, manual_qa] => Ok((
            PathBuf::from(blockers),
            PathBuf::from(readiness),
            PathBuf::from(manual_qa),
        )),
        _ => Err(USAGE.to_string()),
    }
}

pub(super) fn required_blockers(text: &str) -> Result<Vec<String>, String> {
    text.split("## Required Evidence")
        .nth(1)
        .map(parse_required_section)
        .filter(|items| !items.is_empty())
        .ok_or_else(|| "paid beta readiness is missing Required Evidence bullets".to_string())
}

pub(super) fn blocker_statuses(text: &str) -> BTreeMap<String, &'static str> {
    markdown_rows(text)
        .into_iter()
        .filter(|cells| cells.len() == 5 && !cells[0].starts_with("---") && cells[0] != "Blocker")
        .filter_map(|cells| match cells[1].as_str() {
            "Blocked" => Some((cells[0].clone(), "Blocked")),
            "Verified" => Some((cells[0].clone(), "Verified")),
            _ => None,
        })
        .collect()
}

fn default_paths() -> (PathBuf, PathBuf, PathBuf) {
    (
        PathBuf::from("docs/release-blockers.md"),
        PathBuf::from("docs/paid-beta-readiness.md"),
        PathBuf::from("docs/manual-qa.md"),
    )
}

fn parse_required_section(section: &str) -> Vec<String> {
    section
        .lines()
        .skip_while(|line| !line.trim().starts_with("- "))
        .take_while(|line| line.trim().starts_with("- "))
        .filter_map(|line| line.trim().strip_prefix("- "))
        .map(str::to_string)
        .collect()
}

fn markdown_rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
