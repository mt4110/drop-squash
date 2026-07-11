#[derive(Clone, Copy)]
pub(super) enum Kind {
    Artifact,
    Website,
    Refund,
    Checkout,
    GitHubRelease,
    HomebrewPullRequest,
}

pub(super) fn is_valid(kind: Kind, value: &str) -> bool {
    value.starts_with("https://") && has_no_whitespace(value) && matches_kind(kind, value)
}

fn matches_kind(kind: Kind, value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    match kind {
        Kind::Artifact => {
            has_github_release_asset(
                value,
                "https://github.com/mt4110/drop-squash/releases/download/",
            ) && lower.ends_with(".dmg")
        }
        Kind::Website => {
            has_release_status_path(&lower)
                && !lower.contains("lemonsqueezy.com")
                && !lower.contains("checkout")
        }
        Kind::Refund => has_refund_path(&lower) && !has_store_or_checkout(&lower),
        Kind::Checkout => lower.contains("lemonsqueezy.com") && has_checkout_buy_id(&lower),
        Kind::GitHubRelease => {
            has_release_tag_suffix(value, "https://github.com/mt4110/drop-squash/releases/tag/")
        }
        Kind::HomebrewPullRequest => {
            has_numeric_suffix(value, "https://github.com/mt4110/homebrew-tap/pull/")
        }
    }
}

fn has_refund_path(lower: &str) -> bool {
    lower.ends_with("/refund") || lower.ends_with("/refund/")
}

fn has_store_or_checkout(lower: &str) -> bool {
    lower.contains("lemonsqueezy.com") || lower.contains("checkout")
}

fn has_checkout_buy_id(lower: &str) -> bool {
    lower
        .split("/checkout/buy/")
        .nth(1)
        .is_some_and(|value| !value.is_empty())
}

fn has_release_status_path(lower: &str) -> bool {
    lower.ends_with("/release-status") || lower.ends_with("/release-status/")
}

fn has_numeric_suffix(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
    })
}

fn has_release_tag_suffix(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        suffix.starts_with('v')
            && suffix.chars().any(|value| value == '.')
            && suffix
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '-' | '_'))
    })
}

fn has_github_release_asset(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        let parts = suffix.split('/').collect::<Vec<_>>();
        parts.len() == 2 && parts.iter().all(|part| !part.is_empty())
    })
}

fn has_no_whitespace(value: &str) -> bool {
    !value.chars().any(char::is_whitespace)
}
