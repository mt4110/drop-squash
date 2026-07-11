#[derive(Clone, Copy)]
pub(super) enum Kind {
    Artifact,
    Website,
    Checkout,
    GitHubRelease,
    HomebrewPullRequest,
}

pub(super) fn is_valid(kind: Kind, value: &str) -> bool {
    value.starts_with("https://") && matches_kind(kind, value)
}

fn matches_kind(kind: Kind, value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    match kind {
        Kind::Artifact => {
            value.starts_with("https://github.com/mt4110/drop-squash/releases/download/")
                && lower.ends_with(".dmg")
        }
        Kind::Website => {
            has_release_status_path(&lower)
                && !lower.contains("lemonsqueezy.com")
                && !lower.contains("checkout")
        }
        Kind::Checkout => lower.contains("lemonsqueezy.com") && lower.contains("/checkout/buy/"),
        Kind::GitHubRelease => {
            value.starts_with("https://github.com/mt4110/drop-squash/releases/tag/")
        }
        Kind::HomebrewPullRequest => {
            has_numeric_suffix(value, "https://github.com/mt4110/homebrew-tap/pull/")
        }
    }
}

fn has_release_status_path(lower: &str) -> bool {
    lower.ends_with("/release-status") || lower.ends_with("/release-status/")
}

fn has_numeric_suffix(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
    })
}
