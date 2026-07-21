const STRONG: [&str; 7] = [
    "leak-zero",
    "audit-ready",
    "pii-safe",
    "enterprise-safe",
    "complete automatic pii protection",
    "completed selective masking",
    "enterprise audit readiness",
];

const NEGATED: [&str; 11] = [
    "not ",
    "no ",
    "do not",
    "does not",
    "must not",
    "never ",
    "unsafe claims",
    "rejecting overclaims",
    "notyetclaimed",
    "not yet claimed",
    "without claiming",
];

pub(super) fn check_line(line_number: usize, previous: &str, line: &str) -> Result<(), String> {
    let lower = line.to_ascii_lowercase();
    let context = format!("{} {lower}", previous.to_ascii_lowercase());
    for phrase in STRONG {
        if lower.contains(phrase) && !is_negated(&context) {
            return Err(format!(
                "line {line_number}: strong claim phrase {phrase:?} lacks negation"
            ));
        }
    }
    Ok(())
}

fn is_negated(line: &str) -> bool {
    NEGATED.iter().any(|marker| line.contains(marker))
}
