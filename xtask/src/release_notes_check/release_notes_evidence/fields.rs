#[cfg(test)]
mod blockers;
#[cfg(test)]
mod tests;

use super::url;

pub(super) const URL: [(&str, url::Kind); 7] = [
    ("Artifact URL", url::Kind::Artifact),
    ("Public website URL", url::Kind::Website),
    ("Pricing URL", url::Kind::Pricing),
    ("Refund policy URL", url::Kind::Refund),
    ("Live checkout URL", url::Kind::Checkout),
    ("GitHub Release URL", url::Kind::GitHubRelease),
    ("Homebrew tap PR URL", url::Kind::HomebrewPullRequest),
];

pub(super) const EVIDENCE: [&str; 23] = [
    "`codesign`",
    "`spctl`",
    "`stapler`",
    "Apple notary log",
    "Gatekeeper clean-machine open",
    "`docs/release-blockers.md` status",
    "Manual QA record",
    "Conversion safety evidence",
    "Queue evidence",
    "Trash source policy",
    "Lemon Squeezy product setup",
    "Lemon Squeezy sandbox purchase",
    "Valid sandbox activation",
    "Empty key activation",
    "Invalid license key handling",
    "License network failure",
    "Expired license refresh",
    "Local license forget",
    "GitHub Release checksum",
    "Homebrew tap PR",
    "Homebrew install result",
    "Known limitations",
    "Support contact",
];
