use super::EXPECTED_RECORD_TARGETS;

pub(super) fn matches_record_target(blocker: &str, reference: &str) -> bool {
    let Some((_, target)) = EXPECTED_RECORD_TARGETS
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
    else {
        return false;
    };
    match *target {
        "`docs/manual-qa.md`" => reference == "`docs/manual-qa.md`",
        "`https://...`" if blocker == "Public website deployment" => is_public_website(reference),
        "`https://...`" if blocker == "Refund policy finalized" => is_refund_policy(reference),
        "`https://...`" if blocker == "Live checkout link" => is_live_checkout(reference),
        "`https://...`" => reference.starts_with("https://"),
        "GitHub Release" => has_expected_url(
            reference,
            "GitHub Release",
            "https://github.com/mt4110/drop-squash/releases/tag/",
        ),
        "Homebrew tap PR" => has_expected_url(
            reference,
            "Homebrew tap PR",
            "https://github.com/mt4110/homebrew-tap/pull/",
        ),
        other => reference == other,
    }
}

fn is_public_website(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && has_release_status_path(&lower)
        && !lower.contains("lemonsqueezy.com")
        && !lower.contains("checkout")
}

fn has_release_status_path(lower: &str) -> bool {
    lower.ends_with("/release-status") || lower.ends_with("/release-status/")
}

fn is_live_checkout(reference: &str) -> bool {
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

fn is_refund_policy(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    reference.starts_with("https://")
        && (lower.ends_with("/refund") || lower.ends_with("/refund/"))
        && !lower.contains("lemonsqueezy.com")
        && !lower.contains("checkout")
}

fn has_expected_url(reference: &str, label: &str, prefix: &str) -> bool {
    if !reference.starts_with(label) || super::super::placeholders::has_token(reference) {
        return false;
    }
    let matches = if label == "Homebrew tap PR" {
        has_numeric_suffix
    } else {
        has_release_tag_suffix
    };
    super::super::reference_urls::single(reference).is_some_and(|url| matches(url, prefix))
}

fn has_numeric_suffix(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
    })
}

fn has_release_tag_suffix(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(is_v_semver)
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
