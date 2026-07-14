pub(super) fn for_blocker(blocker: &str) -> Option<&'static [&'static str]> {
    match blocker {
        "Lemon Squeezy product setup" => Some(&["intended product", "DropSquash", "license keys"]),
        "Lemon Squeezy sandbox purchase" => Some(&[
            "sandbox checkout",
            "intended product",
            "test buyer",
            "order",
        ]),
        "Public website deployment" => Some(&[
            "production site",
            "dropsquash.app",
            "release-status",
            "privacy",
            "pricing",
            "license",
            "support",
            "download",
        ]),
        "Pricing finalized" => Some(&[
            "final pricing page URL",
            "dropsquash.app",
            "no draft price copy",
            "checkout goes live",
        ]),
        "Refund policy finalized" => Some(&[
            "final refund policy URL",
            "dropsquash.app",
            "linked",
            "checkout goes live",
        ]),
        "Live checkout link" => Some(&[
            "public pricing page",
            "live",
            "store.lemonsqueezy.com/checkout/buy/<id>",
            "tested Lemon Squeezy checkout",
            "intended product",
        ]),
        _ => None,
    }
}
