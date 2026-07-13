pub(super) const MAPPING: &[(&str, &[&str])] = &[
    ("Packaged macOS manual QA", &["Manual QA record"]),
    (
        "Lemon Squeezy product setup",
        &["Lemon Squeezy product setup"],
    ),
    (
        "Lemon Squeezy sandbox purchase",
        &["Lemon Squeezy sandbox purchase"],
    ),
    ("Empty key activation", &["Empty key activation"]),
    ("Valid sandbox activation", &["Valid sandbox activation"]),
    (
        "Invalid license key handling",
        &["Invalid license key handling"],
    ),
    ("License network failure", &["License network failure"]),
    ("Expired license refresh", &["Expired license refresh"]),
    ("Local license forget", &["Local license forget"]),
    ("Public website deployment", &["Public website URL"]),
    ("Refund policy finalized", &["Refund policy URL"]),
    ("Live checkout link", &["Live checkout URL"]),
    ("Signed DMG", &["`codesign`"]),
    (
        "Notarized and stapled DMG",
        &["`spctl`", "`stapler`", "Apple notary log"],
    ),
    (
        "Gatekeeper clean-machine open",
        &["Gatekeeper clean-machine open"],
    ),
    (
        "Benchmark release set",
        &["Benchmark sample set", "Benchmark regression threshold"],
    ),
    ("Published checksum", &["GitHub Release checksum"]),
    (
        "Homebrew cask install",
        &["Homebrew tap PR", "Homebrew install result"],
    ),
];

pub(super) fn has_mapping(blocker: &str) -> bool {
    MAPPING.iter().any(|(mapped, _)| *mapped == blocker)
}
