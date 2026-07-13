const DISALLOWED: [&str; 48] = [
    "-----begin ",
    "private key-----",
    "apple api issuer:",
    "apple api key:",
    "apple_api_issuer=",
    "apple_api_issuer:",
    "apple_api_key=",
    "apple_api_key:",
    "apple_certificate=",
    "apple_certificate_password=",
    "apple_certificate_password:",
    "apple id:",
    "apple_id=",
    "apple_id:",
    "apple_password=",
    "apple_password:",
    "apple team id:",
    "apple_team_id=",
    "apple_team_id:",
    "lemon squeezy api key:",
    "lemon_squeezy_api_key=",
    "lemon_squeezy_api_key:",
    "lemon_squeezy_product_id=",
    "lemon_squeezy_product_id:",
    "lemon_squeezy_store_id=",
    "lemon_squeezy_store_id:",
    "lemon_squeezy_variant_id=",
    "lemon_squeezy_variant_id:",
    "license_key=",
    "license_key:",
    "license key=",
    "license key:",
    "product id:",
    "product_id=",
    "product_id:",
    "raw key=",
    "raw key:",
    "raw license key=",
    "raw license key:",
    "raw_license_key=",
    "raw_license_key:",
    "raw_key:",
    "store id:",
    "store_id=",
    "store_id:",
    "variant id:",
    "variant_id=",
    "variant_id:",
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
mod tests;
