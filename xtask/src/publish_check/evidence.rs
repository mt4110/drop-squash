use super::urls;

pub(super) fn matches(blocker: &str, reference: &str) -> bool {
    let reference = reference.trim().trim_matches('`');
    if reference.is_empty() || has_placeholder_token(reference) {
        return false;
    }
    match blocker {
        "Signed DMG" | "Notarized and stapled DMG" => reference == "Release notes",
        "Published checksum" => has_release_tag(reference),
        "Homebrew cask install" => has_homebrew_pr(reference),
        "Public website deployment" => is_public_site(reference),
        "Refund policy finalized" => is_refund(reference),
        "Live checkout link" => is_checkout(reference),
        _ => reference == "docs/manual-qa.md",
    }
}

fn has_placeholder_token(reference: &str) -> bool {
    reference
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| matches!(token.to_ascii_lowercase().as_str(), "tbd" | "todo"))
}

fn is_public_site(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && (lower.ends_with("/release-status") || lower.ends_with("/release-status/"))
        && !lower.contains("lemonsqueezy.com")
        && !lower.contains("checkout")
}

fn is_refund(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && (lower.ends_with("/refund") || lower.ends_with("/refund/"))
        && !lower.contains("lemonsqueezy.com")
        && !lower.contains("checkout")
}

fn is_checkout(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && !reference.chars().any(char::is_whitespace)
        && lower.contains("lemonsqueezy.com")
        && lower
            .split("/checkout/buy/")
            .nth(1)
            .is_some_and(has_single_path_segment)
}

fn has_single_path_segment(value: &str) -> bool {
    let id = value.split('?').next().unwrap_or(value);
    !id.is_empty() && !id.contains('/')
}

fn has_release_tag(reference: &str) -> bool {
    reference.starts_with("GitHub Release ")
        && urls::single_https(reference).is_some_and(|part| {
            part.strip_prefix("https://github.com/mt4110/drop-squash/releases/tag/")
                .is_some_and(is_v_semver)
        })
}

fn is_v_semver(value: &str) -> bool {
    let Some(version) = value.strip_prefix('v') else {
        return false;
    };
    let parts = version.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|value| value.is_ascii_digit()))
}

fn has_homebrew_pr(reference: &str) -> bool {
    reference.starts_with("Homebrew tap PR ")
        && urls::single_https(reference).is_some_and(|part| {
            part.strip_prefix("https://github.com/mt4110/homebrew-tap/pull/")
                .is_some_and(|suffix| {
                    !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
                })
        })
}
