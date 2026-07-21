pub(crate) fn distribution_lines(text: &str) -> Vec<String> {
    let pending = super::pending_rows::pending_rows(text);
    let groups = super::section::grouped(&pending, Some("distribution"));
    super::distribution::extra_lines(text, &groups)
}

pub(crate) fn license_lines(text: &str) -> Vec<String> {
    let pending = super::pending_rows::pending_rows(text);
    let groups = super::section::grouped(&pending, Some("license"));
    super::license::extra_lines(text, &groups)
}

pub(crate) fn license_ready_lines(text: &str) -> Vec<String> {
    license_lines(text)
        .into_iter()
        .filter(|line| !is_ready_duplicate(line))
        .collect()
}

pub(crate) fn license_sandbox_rows() -> [&'static str; 6] {
    super::license_candidates::sandbox_lines()
}

fn is_ready_duplicate(line: &str) -> bool {
    [
        "license fresh build command:",
        "license fresh app artifact:",
        "license cache inspect command:",
        "license cache helper command:",
        "license launch note:",
        "license fresh app launch command:",
        "license fresh app network failure launch command:",
        "license product setup row candidate:",
        "license product setup markdown row:",
        "license sandbox purchase row candidate:",
        "license sandbox purchase markdown row:",
        "license valid activation row candidate:",
        "license valid activation markdown row:",
        "license empty activation row candidate:",
        "license empty activation markdown row:",
        "license invalid activation row candidate:",
        "license invalid activation markdown row:",
        "license network failure row candidate:",
        "license network failure markdown row:",
        "license expired refresh row candidate:",
        "license expired refresh markdown row:",
        "license forget helper command:",
        "license forget command:",
        "license activation before-state command:",
        "license activation evidence reminder:",
    ]
    .iter()
    .any(|prefix| line.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::{distribution_lines, license_lines, license_ready_lines};

    #[test]
    fn includes_distribution_preflight_lines() {
        let text = "| App artifact | /tmp/work/target/release/bundle/dmg/DropSquash.dmg |\n| Output folder | /tmp/out |\n| Codesign verification | expected |  |\n";
        let lines = distribution_lines(text);
        assert!(lines
            .iter()
            .any(|line| line.starts_with("distribution fresh build command:")));
        assert!(lines
            .iter()
            .any(|line| line.starts_with("distribution artifact check:")));
    }

    #[test]
    fn includes_license_helper_lines() {
        let text = "| App artifact | /tmp/DropSquash.dmg |\n| Config path | /tmp/state/config.json |\n| History path | /tmp/state/history.jsonl |\n| License cache path | /tmp/state/license.json |\n| Valid sandbox activation | expected |  |\n";
        let lines = license_lines(text);
        assert!(lines
            .iter()
            .any(|line| line.starts_with("license fresh build command:")));
        assert!(lines
            .iter()
            .any(|line| line.starts_with("license cache helper command:")));
    }

    #[test]
    fn drops_duplicate_ready_license_lines() {
        let text = "| App artifact | /tmp/DropSquash.dmg |\n| Config path | /tmp/state/config.json |\n| History path | /tmp/state/history.jsonl |\n| License cache path | /tmp/state/license.json |\n| Valid sandbox activation | expected |  |\n";
        let lines = license_ready_lines(text);
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license fresh build command:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license cache helper command:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license empty activation row candidate:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license invalid activation row candidate:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license network failure row candidate:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license forget helper command:")));
        assert!(!lines
            .iter()
            .any(|line| line.starts_with("license activation before-state command:")));
        assert!(lines
            .iter()
            .any(|line| line.starts_with("license diagnostics command:")));
        assert!(lines
            .iter()
            .any(|line| line.starts_with("license valid activation evidence reminder:")));
    }
}
