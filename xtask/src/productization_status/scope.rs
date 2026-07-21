#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct Scope {
    deferred: Vec<String>,
}

impl Scope {
    #[cfg(test)]
    pub(super) fn from_deferred(deferred: Vec<&str>) -> Self {
        Self {
            deferred: deferred.into_iter().map(str::to_string).collect(),
        }
    }

    pub(super) fn is_deferred(&self, blocker: &str) -> bool {
        self.deferred
            .iter()
            .any(|entry| entry == blocker || entry.starts_with(blocker))
    }

    pub(super) fn deferred(&self) -> &[String] {
        &self.deferred
    }
}

pub(super) fn load() -> Result<Scope, String> {
    let path = "docs/paid-beta-readiness.md";
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("failed to read {path}: {error}"))?;
    Ok(parse(&text))
}

fn parse(text: &str) -> Scope {
    let deferred = text
        .split("## Deliberately Deferred Until After The Testable Paid Beta")
        .nth(1)
        .and_then(|tail| tail.split("## Out Of Scope").next())
        .map(bullets)
        .unwrap_or_default();
    Scope { deferred }
}

fn bullets(section: &str) -> Vec<String> {
    let mut bullets = Vec::new();
    let mut started = false;
    for line in section.lines() {
        if let Some(value) = line.trim().strip_prefix("- ") {
            let value = value.trim();
            if !value.is_empty() {
                bullets.push(value.to_string());
                started = true;
            }
            continue;
        }
        if started {
            break;
        }
    }
    bullets
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_deferred_paid_beta_blockers() {
        let scope = parse(
            "\
## Deliberately Deferred Until After The Testable Paid Beta

- Public website deployment
- Pricing finalized
- Published checksum

## Out Of Scope Until Market Validation
",
        );

        assert!(scope.is_deferred("Public website deployment"));
        assert!(scope.is_deferred("Pricing finalized"));
        assert!(scope.is_deferred("Published checksum"));
        assert!(!scope.is_deferred("Signed DMG"));
    }

    #[test]
    fn stops_after_first_bullet_block() {
        let scope = parse(
            "\
## Deliberately Deferred Until After The Testable Paid Beta

- Public website deployment
- Pricing finalized

Current deferred public-web status as of Saturday, July 18, 2026:

- A dropsquash.app -> 162.159.143.30

## Out Of Scope Until Market Validation
",
        );

        assert!(scope.is_deferred("Public website deployment"));
        assert!(scope.is_deferred("Pricing finalized"));
        assert!(!scope.is_deferred("A dropsquash.app -> 162.159.143.30"));
    }
}
