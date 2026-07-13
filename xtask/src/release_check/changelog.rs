use std::path::Path;

pub(super) fn check(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let missing = missing_requirements(&text);
    if missing.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} is missing changelog release requirements: {}",
        path.display(),
        missing.join(", ")
    ))
}

fn missing_requirements(text: &str) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if !text.contains("# Changelog") {
        missing.push("# Changelog");
    }
    if !text.contains("## Unreleased") {
        missing.push("## Unreleased");
    }
    if !has_unreleased_bullet(text) {
        missing.push("Unreleased bullet");
    }
    if has_placeholder(text) {
        missing.push("no placeholders");
    }
    missing
}

fn has_placeholder(text: &str) -> bool {
    text.split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| matches!(token.to_ascii_lowercase().as_str(), "tbd" | "todo"))
}

fn has_unreleased_bullet(text: &str) -> bool {
    text.split("## Unreleased")
        .nth(1)
        .unwrap_or_default()
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .any(|line| line.trim_start().starts_with("- "))
}

#[cfg(test)]
mod tests {
    use super::{check, missing_requirements};

    #[test]
    fn accepts_current_changelog() {
        check(std::path::Path::new("../CHANGELOG.md")).unwrap();
    }

    #[test]
    fn reports_missing_unreleased_bullet() {
        let missing = missing_requirements("# Changelog\n\n## Unreleased\n\n## 0.1.0\n");

        assert!(missing.contains(&"Unreleased bullet"));
    }

    #[test]
    fn reports_placeholder_text() {
        let missing = missing_requirements("# Changelog\n\n## Unreleased\n\n- TODO\n");

        assert!(missing.contains(&"no placeholders"));
    }
}
