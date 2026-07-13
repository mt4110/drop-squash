const DISALLOWED: [&str; 20] = [
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
    "license key=",
    "license key:",
    "product id:",
    "product_id=",
    "raw key=",
    "raw key:",
    "store id:",
    "store_id=",
    "variant id:",
    "variant_id=",
];

pub(crate) fn violations(scope: &str, text: &str) -> Vec<String> {
    let lower = normalized(text);
    DISALLOWED
        .iter()
        .filter(|marker| lower.contains(**marker))
        .map(|marker| format!("{scope} must not contain secret-like value {marker}"))
        .collect()
}

fn normalized(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    lower
        .replace(" :", ":")
        .replace(": ", ":")
        .replace(" =", "=")
        .replace("= ", "=")
}

#[cfg(test)]
mod tests {
    use super::violations;

    #[test]
    fn rejects_secret_like_assignments() {
        let text = "APPLE_PASSWORD=x LEMON_SQUEEZY_STORE_ID=123 license_key=raw product id: 1";

        let errors = violations("evidence", text);

        assert!(errors.iter().any(|error| error.contains("apple_password")));
        assert!(errors.iter().any(|error| error.contains("store_id")));
        assert!(errors.iter().any(|error| error.contains("license_key")));
        assert!(errors.iter().any(|error| error.contains("product id:")));
    }

    #[test]
    fn rejects_colon_form_store_and_variant_ids() {
        let text = "store id: 123; variant id: 456";

        let errors = violations("evidence", text);

        assert!(errors.iter().any(|error| error.contains("store id:")));
        assert!(errors.iter().any(|error| error.contains("variant id:")));
    }

    #[test]
    fn rejects_secret_like_values_with_spaced_separators() {
        let text = "product_id = 123; store id : 456; license key : raw; raw key = test";

        let errors = violations("evidence", text);

        assert!(errors.iter().any(|error| error.contains("product_id")));
        assert!(errors.iter().any(|error| error.contains("store id:")));
        assert!(errors.iter().any(|error| error.contains("license key:")));
        assert!(errors.iter().any(|error| error.contains("raw key=")));
    }

    #[test]
    fn rejects_license_key_with_spaced_equals() {
        let errors = violations("evidence", "license key = raw-test-key");

        assert!(errors.iter().any(|error| error.contains("license key=")));
    }

    #[test]
    fn allows_safe_evidence_terms() {
        let text =
            "license-key fingerprint exists, private store IDs absent, raw key absent from cache";

        assert!(violations("evidence", text).is_empty());
    }
}
