const DISALLOWED: [&str; 14] = [
    "-----begin ",
    "private key-----",
    "apple_certificate=",
    "apple_certificate_password=",
    "apple_password=",
    "lemon_squeezy_api_key=",
    "lemon_squeezy_product_id=",
    "lemon_squeezy_store_id=",
    "lemon_squeezy_variant_id=",
    "license_key=",
    "license key:",
    "product_id=",
    "store_id=",
    "variant_id=",
];

pub(crate) fn violations(scope: &str, text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    DISALLOWED
        .iter()
        .filter(|marker| lower.contains(**marker))
        .map(|marker| format!("{scope} must not contain secret-like value {marker}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::violations;

    #[test]
    fn rejects_secret_like_assignments() {
        let text = "APPLE_PASSWORD=x LEMON_SQUEEZY_STORE_ID=123 license_key=raw";

        let errors = violations("evidence", text);

        assert!(errors.iter().any(|error| error.contains("apple_password")));
        assert!(errors.iter().any(|error| error.contains("store_id")));
        assert!(errors.iter().any(|error| error.contains("license_key")));
    }

    #[test]
    fn allows_safe_evidence_terms() {
        let text =
            "license-key fingerprint exists, private store IDs absent, raw key absent from cache";

        assert!(violations("evidence", text).is_empty());
    }
}
