use super::urls;

pub(super) fn matches(blocker: &str, reference: &str) -> bool {
    let reference = reference.trim();
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
        _ => reference == "`docs/manual-qa.md`",
    }
}

fn has_placeholder_token(reference: &str) -> bool {
    reference
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| matches!(token.to_ascii_lowercase().as_str(), "tbd" | "todo"))
}

fn is_public_site(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        let path = url.path().to_ascii_lowercase();
        is_release_status_path(&path)
            && !url.host_is_or_subdomain_of("lemonsqueezy.com")
            && !path.contains("checkout")
    })
}

fn is_refund(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        let path = url.path().to_ascii_lowercase();
        is_refund_path(&path)
            && !url.host_is_or_subdomain_of("lemonsqueezy.com")
            && !path.contains("checkout")
    })
}

fn is_checkout(reference: &str) -> bool {
    crate::public_url::HttpsUrl::parse(reference).is_some_and(|url| {
        url.host_is_or_subdomain_of("lemonsqueezy.com")
            && url
                .path()
                .to_ascii_lowercase()
                .strip_prefix("checkout/buy/")
                .is_some_and(has_single_path_segment)
    })
}

fn is_release_status_path(path: &str) -> bool {
    path == "release-status"
        || path.ends_with("/release-status")
        || path.ends_with("/release-status/")
}

fn is_refund_path(path: &str) -> bool {
    path == "refund" || path.ends_with("/refund") || path.ends_with("/refund/")
}

fn has_single_path_segment(value: &str) -> bool {
    let id = value.split('?').next().unwrap_or(value);
    !id.is_empty() && !id.contains('/')
}

fn has_release_tag(reference: &str) -> bool {
    reference.starts_with("GitHub Release ")
        && urls::labeled_https(reference, "GitHub Release").is_some_and(|part| {
            crate::public_url::HttpsUrl::parse(part).is_some_and(|url| {
                url.host_is("github.com")
                    && url
                        .path()
                        .strip_prefix("mt4110/drop-squash/releases/tag/")
                        .is_some_and(is_v_semver)
            })
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
        && urls::labeled_https(reference, "Homebrew tap PR").is_some_and(|part| {
            crate::public_url::HttpsUrl::parse(part).is_some_and(|url| {
                url.host_is("github.com")
                    && url
                        .path()
                        .strip_prefix("mt4110/homebrew-tap/pull/")
                        .is_some_and(|suffix| {
                            !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
                        })
            })
        })
}
