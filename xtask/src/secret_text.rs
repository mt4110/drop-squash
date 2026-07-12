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
