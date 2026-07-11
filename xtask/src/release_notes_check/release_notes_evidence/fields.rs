use super::url;

pub(super) const URL: [(&str, url::Kind); 6] = [
    ("Artifact URL", url::Kind::Artifact),
    ("Public website URL", url::Kind::Website),
    ("Refund policy URL", url::Kind::Refund),
    ("Live checkout URL", url::Kind::Checkout),
    ("GitHub Release URL", url::Kind::GitHubRelease),
    ("Homebrew tap PR URL", url::Kind::HomebrewPullRequest),
];

pub(super) const EVIDENCE: [&str; 19] = [
    "`codesign`",
    "`spctl`",
    "`stapler`",
    "Apple notary log",
    "Gatekeeper clean-machine open",
    "`docs/release-blockers.md` status",
    "Manual QA record",
    "Lemon Squeezy product setup",
    "Lemon Squeezy sandbox purchase",
    "Lemon Squeezy sandbox activation",
    "Empty key activation",
    "Invalid license key handling",
    "License network failure",
    "Local license forget",
    "GitHub Release checksum",
    "Homebrew tap PR",
    "Homebrew install result",
    "Known limitations",
    "Support contact",
];
