pub(super) fn matches_record_target(blocker: &str, reference: &str) -> bool {
    match super::expected_target(blocker) {
        None => false,
        Some("`docs/manual-qa.md`") => reference == "`docs/manual-qa.md`",
        Some("`https://...`") if blocker == "Public website deployment" => {
            is_public_website(reference)
        }
        Some("`https://...`") if blocker == "Refund policy finalized" => {
            is_refund_policy(reference)
        }
        Some("`https://...`") if blocker == "Live checkout link" => is_live_checkout(reference),
        Some("`https://...`") => crate::url_scheme::is_https(reference),
        Some("GitHub Release") => has_expected_url(
            reference,
            "GitHub Release",
            "mt4110/drop-squash/releases/tag/",
        ),
        Some("Homebrew tap PR") => {
            has_expected_url(reference, "Homebrew tap PR", "mt4110/homebrew-tap/pull/")
        }
        Some(other) => reference == other,
    }
}

fn is_public_website(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        let path = url.path().to_ascii_lowercase();
        has_release_status_path(&path)
            && !url.host_is_or_subdomain_of("lemonsqueezy.com")
            && !path.contains("checkout")
    })
}

fn has_release_status_path(lower: &str) -> bool {
    lower == "release-status"
        || lower.ends_with("/release-status")
        || lower.ends_with("/release-status/")
}

fn is_live_checkout(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        url.host_is_or_subdomain_of("lemonsqueezy.com")
            && url
                .path()
                .to_ascii_lowercase()
                .split("checkout/buy/")
                .nth(1)
                .is_some_and(has_single_path_segment)
    })
}

fn has_single_path_segment(value: &str) -> bool {
    let id = value.split('?').next().unwrap_or(value);
    !id.is_empty() && !id.contains('/')
}

fn is_refund_policy(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        let path = url.path().to_ascii_lowercase();
        (path == "refund" || path.ends_with("/refund") || path.ends_with("/refund/"))
            && !url.host_is_or_subdomain_of("lemonsqueezy.com")
            && !path.contains("checkout")
    })
}

fn has_expected_url(reference: &str, label: &str, prefix: &str) -> bool {
    if !reference.starts_with(label) || super::super::placeholders::has_token(reference) {
        return false;
    }
    let suffix_matches = if label == "Homebrew tap PR" {
        is_numeric
    } else {
        is_v_semver
    };
    super::super::reference_urls::single(reference).is_some_and(|value| {
        crate::public_url::HttpsUrl::parse(value).is_some_and(|url| {
            url.host_is("github.com") && url.path().strip_prefix(prefix).is_some_and(suffix_matches)
        })
    })
}

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|value| value.is_ascii_digit())
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
