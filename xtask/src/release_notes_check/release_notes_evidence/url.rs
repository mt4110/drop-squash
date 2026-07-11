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
        Kind::Website => !lower.contains("lemonsqueezy.com") && !lower.contains("checkout"),
        Kind::Checkout => lower.contains("lemonsqueezy.com") && lower.contains("checkout"),
        Kind::GitHubRelease => {
            value.starts_with("https://github.com/mt4110/drop-squash/releases/tag/")
        }
        Kind::HomebrewPullRequest => {
            value.starts_with("https://github.com/mt4110/homebrew-tap/pull/")
        }
    }
}
