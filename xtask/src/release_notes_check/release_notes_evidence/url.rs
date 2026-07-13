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
    crate::public_url::HttpsUrl::parse(value)
        .is_some_and(|url| !url.has_query_or_fragment() && matches_kind(kind, &url))
}

fn matches_kind(kind: Kind, url: &crate::public_url::HttpsUrl<'_>) -> bool {
    let path = url.path().to_ascii_lowercase();
    match kind {
        Kind::Artifact => {
            url.host_is("github.com")
                && has_github_release_asset(url.path(), "mt4110/drop-squash/releases/download/")
                && path.ends_with(".dmg")
        }
        Kind::Website => {
            url.host_is("dropsquash.app")
                && has_release_status_path(&path)
                && !url.host_is_or_subdomain_of("lemonsqueezy.com")
                && !path.contains("checkout")
        }
        Kind::Refund => {
            url.host_is("dropsquash.app")
                && has_refund_path(&path)
                && !has_store_or_checkout(url, &path)
        }
        Kind::Checkout => {
            url.host_is("store.lemonsqueezy.com") && crate::public_url::has_checkout_buy_path(&path)
        }
        Kind::GitHubRelease => {
            url.host_is("github.com")
                && has_release_tag_suffix(url.path(), "mt4110/drop-squash/releases/tag/")
        }
        Kind::HomebrewPullRequest => {
            url.host_is("github.com") && has_numeric_suffix(url.path(), "mt4110/homebrew-tap/pull/")
        }
    }
}

fn has_refund_path(lower: &str) -> bool {
    lower == "refund" || lower == "refund/"
}

fn has_store_or_checkout(url: &crate::public_url::HttpsUrl<'_>, lower_path: &str) -> bool {
    url.host_is_or_subdomain_of("lemonsqueezy.com") || lower_path.contains("checkout")
}

fn has_release_status_path(lower: &str) -> bool {
    lower == "release-status" || lower == "release-status/"
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

fn has_github_release_asset(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(|suffix| {
        let parts = suffix.split('/').collect::<Vec<_>>();
        parts.len() == 2 && parts.iter().all(|part| !part.is_empty())
    })
}
