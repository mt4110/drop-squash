const DISALLOWED: [&str; 7] = [
    "-----begin ",
    "private key-----",
    "apple_certificate=",
    "apple_certificate_password=",
    "apple_password=",
    "lemon_squeezy_api_key=",
    "license key:",
];

pub(crate) fn violations(scope: &str, text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    DISALLOWED
        .iter()
        .filter(|marker| lower.contains(**marker))
        .map(|marker| format!("{scope} must not contain secret-like value {marker}"))
        .collect()
}
