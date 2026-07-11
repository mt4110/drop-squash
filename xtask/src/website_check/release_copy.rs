use std::path::Path;

pub(super) fn check(root: &Path, errors: &mut Vec<String>) {
    require_page_text(root, "index.html", "Release status", errors);
    require_page_text(root, "download.html", "macOS beta", errors);
    require_page_text(root, "download.html", "DropSquash.dmg", errors);
    require_page_text(root, "download.html", "notarization", errors);
    require_page_text(root, "download.html", "checksum", errors);
    require_page_text(root, "pricing.html", "Checkout opens after", errors);
    require_page_text(root, "pricing.html", "signed beta release", errors);
    require_page_text(
        root,
        "pricing.html",
        "10 successful conversions are free",
        errors,
    );
    require_page_text(
        root,
        "pricing.html",
        "Failed or cancelled conversions do not count",
        errors,
    );
    require_page_text(root, "privacy.html", "does not upload media", errors);
    require_page_text(root, "privacy.html", "Telemetry is off by default", errors);
    require_page_text(root, "privacy.html", "privacy receipts", errors);
    require_page_text(root, "privacy.html", "uploaded_bytes = 0", errors);
    require_page_text(
        root,
        "privacy.html",
        "License activation contacts Lemon Squeezy",
        errors,
    );
    require_page_text(root, "license.html", "license-key fingerprint", errors);
    require_page_text(
        root,
        "license.html",
        "does not persist the raw license key",
        errors,
    );
    require_page_text(root, "license.html", "local license cache", errors);
    require_page_text(root, "license.html", "Offline grace", errors);
    require_page_text(
        root,
        "support.html",
        "Do not send screen recordings",
        errors,
    );
    require_page_text(root, "support.html", "FAQ", errors);
    require_page_text(root, "support.html", "What is a privacy receipt?", errors);
    require_page_text(
        root,
        "support.html",
        "Does DropSquash upload my videos?",
        errors,
    );
    require_page_text(root, "support.html", "Does it use ffmpeg?", errors);
    require_page_text(root, "support.html", "app version", errors);
    require_page_text(root, "refund.html", "draft policy", errors);
    require_page_text(root, "refund.html", "checkout goes live", errors);
    require_page_text(root, "refund.html", "cannot activate", errors);
    require_page_text(
        root,
        "refund.html",
        "basic local conversion workflow",
        errors,
    );
    require_page_text(root, "refund.html", "order email", errors);
    require_page_text(root, "refund.html", "Lemon Squeezy order flow", errors);
}

fn require_page_text(root: &Path, page: &str, needle: &str, errors: &mut Vec<String>) {
    let path = root.join(page);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return;
    };
    if !text.contains(needle) {
        errors.push(format!("{page} is missing required text: {needle}"));
    }
}
