use super::{distribution_candidates, license_candidates, pending_rows, section};

pub(super) fn for_filter(text: &str, filter: &str) -> Vec<String> {
    let pending = pending_rows::pending_rows(text)
        .into_iter()
        .map(|(label, _)| label)
        .collect::<Vec<_>>();
    let lines = match filter {
        "license" => license_candidates::lines().to_vec(),
        "distribution" => distribution_candidates::lines().to_vec(),
        _ => return Vec::new(),
    };
    lines
        .into_iter()
        .filter(|line| line.contains("markdown row:"))
        .filter(|line| {
            pending.iter().any(|label| {
                section::filter_for_label(label) == filter && line.contains(&format!("| {label} |"))
            })
        })
        .map(str::to_string)
        .collect()
}
