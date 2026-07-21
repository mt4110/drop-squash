const WEAK_EVIDENCE: [&str; 4] = ["pending", "unproven", "untested", "proof absent"];

pub(super) fn check(path: &str, source: Option<&str>, evidence: &str) -> Result<(), String> {
    if !matches!(source, Some("covered" | "fail-closed")) {
        return Ok(());
    }
    let lower = evidence.to_ascii_lowercase();
    for marker in WEAK_EVIDENCE {
        if lower.contains(marker) {
            return Err(format!(
                "{path} cannot be {source:?} while evidence says {marker:?}"
            ));
        }
    }
    Ok(())
}
