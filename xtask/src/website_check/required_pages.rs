use std::path::Path;

const REQUIRED: [&str; 9] = [
    "index.html",
    "release-status/index.html",
    "download.html",
    "pricing.html",
    "terms.html",
    "privacy.html",
    "support.html",
    "license.html",
    "refund.html",
];

pub(super) fn check(root: &Path, errors: &mut Vec<String>) {
    for page in REQUIRED {
        if !root.join(page).is_file() {
            errors.push(format!("website is missing required page: {page}"));
        }
    }
}
