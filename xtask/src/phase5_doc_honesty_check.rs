mod claim_words;

const DEFAULT_DOCS: [&str; 10] = [
    "README.md",
    "docs/phase5-alpha.md",
    "docs/build-week-submission.md",
    "docs/build-week-judge-runbook.md",
    "docs/build-week-devpost-draft.md",
    "docs/build-week-devpost-fields.md",
    "docs/build-week-devpost-checklist.md",
    "docs/build-week-final-entry-sheet.md",
    "docs/build-week-submit-packet.md",
    "docs/build-week-demo-script.md",
];
const WEAK: [&str; 4] = ["pending", "unproven", "untested", "proof absent"];

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [] => check_default()?,
        [path] => check_path(path)?,
        _ => return Err("usage: cargo run -p xtask -- phase5-doc-honesty-check [doc.md]".into()),
    }
    println!("Phase 5 doc honesty checks passed");
    Ok(())
}

pub(crate) fn check_default() -> Result<(), String> {
    for path in DEFAULT_DOCS {
        check_path(path)?;
    }
    Ok(())
}

fn check_path(path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    check(&text).map_err(|error| format!("{path}: {error}"))
}

fn check(text: &str) -> Result<(), String> {
    let mut table = Table::Other;
    let mut previous = "";
    let mut negated_block_lines = 0usize;
    for (index, line) in text.lines().enumerate() {
        if line.to_ascii_lowercase().contains("unsafe claims") {
            negated_block_lines = 20;
        }
        let context = if negated_block_lines > 0 {
            "unsafe claims"
        } else {
            previous
        };
        claim_words::check_line(index + 1, context, line)?;
        negated_block_lines = negated_block_lines.saturating_sub(1);
        if line.contains("| Exposure path | Required signal | Phase 5 status |") {
            table = Table::Coverage;
            continue;
        }
        if line.contains("| Exposure path | Classification | Strongest current evidence |") {
            table = Table::Evidence;
            continue;
        }
        if !line.starts_with('|') || line.starts_with("| ---") {
            previous = line;
            continue;
        }
        check_row(index + 1, line, table)?;
        previous = line;
    }
    Ok(())
}

fn check_row(line_number: usize, line: &str, table: Table) -> Result<(), String> {
    let cells: Vec<_> = line.split('|').map(str::trim).collect();
    let (claim, evidence) = match table {
        Table::Coverage => (cells.get(3), cells.get(3)),
        Table::Evidence => (cells.get(2), cells.get(3)),
        Table::Other => return Ok(()),
    };
    let Some(claim) = claim else { return Ok(()) };
    if !is_strong_claim(claim) {
        return Ok(());
    }
    let evidence = evidence.copied().unwrap_or_default().to_ascii_lowercase();
    for weak in WEAK {
        if evidence.contains(weak) {
            return Err(format!(
                "line {line_number}: strong Phase 5 claim contains weak evidence marker {weak:?}"
            ));
        }
    }
    Ok(())
}

fn is_strong_claim(claim: &str) -> bool {
    let lower = claim.to_ascii_lowercase();
    if lower.contains("detected-but-not-covered") || lower.contains("detected_but_not_covered") {
        return false;
    }
    lower.contains("covered")
        || lower.contains("fail-closed")
        || lower.contains("fail_closed")
        || lower.contains("code-covered")
}

#[derive(Clone, Copy)]
enum Table {
    Other,
    Coverage,
    Evidence,
}

#[cfg(test)]
mod tests;
