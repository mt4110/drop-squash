const COLUMN_COUNT: usize = 5;
const BLOCKER_COLUMN: usize = 0;
const STATUS_COLUMN: usize = 1;
const COMPLETION_EVIDENCE_COLUMN: usize = 2;
const EVIDENCE_REFERENCE_COLUMN: usize = 3;
const RECORD_IN_COLUMN: usize = 4;

pub(crate) fn find<'a>(text: &'a str, blocker: &str) -> Option<&'a str> {
    text.lines().find(|line| {
        cells(line).and_then(|cells| cells.get(BLOCKER_COLUMN).copied()) == Some(blocker)
    })
}

pub(crate) fn has_status(line: &str, blocker: &str, status: &str) -> bool {
    let Some(cells) = cells(line) else {
        return false;
    };
    cells.get(BLOCKER_COLUMN) == Some(&blocker) && cells.get(STATUS_COLUMN) == Some(&status)
}

pub(super) fn evidence_reference(line: &str) -> Option<&str> {
    cells(line).and_then(|cells| cells.get(EVIDENCE_REFERENCE_COLUMN).copied())
}

pub(super) fn completion_evidence(line: &str) -> Option<&str> {
    cells(line).and_then(|cells| cells.get(COMPLETION_EVIDENCE_COLUMN).copied())
}

pub(super) fn record_in(line: &str) -> Option<&str> {
    cells(line).and_then(|cells| cells.get(RECORD_IN_COLUMN).copied())
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
    (cells.len() == COLUMN_COUNT).then_some(cells)
}
