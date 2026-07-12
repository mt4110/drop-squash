use super::evidence;

pub(super) fn unverified_blockers(text: &str) -> Vec<&'static str> {
    crate::release_check::required_blockers()
        .iter()
        .copied()
        .filter(|blocker| !is_verified(text, blocker))
        .collect()
}

pub(super) fn unverified_blockers_error(blockers: &[&str]) -> String {
    format!(
        "release blockers must be Verified with traceable Evidence reference before publish: {}",
        blockers.join(", ")
    )
}

fn is_verified(text: &str, blocker: &str) -> bool {
    text.lines().any(|line| {
        let Some(cells) = cells(line) else {
            return false;
        };
        cells.first() == Some(&blocker)
            && cells.get(1) == Some(&"Verified")
            && cells
                .get(3)
                .is_some_and(|reference| evidence::matches(blocker, reference))
    })
}

fn cells(line: &str) -> Option<Vec<&str>> {
    if !line.starts_with('|') {
        return None;
    }
    let cells = line
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    (cells.len() == 5).then_some(cells)
}
