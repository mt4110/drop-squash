use super::violations;

#[test]
fn rejects_secret_like_assignments() {
    let text = "APPLE_PASSWORD=x LEMON_SQUEEZY_STORE_ID=123 license_key=raw product id: 1 APPLE_API_KEY=ABCDEF1234";

    let errors = violations("evidence", text);

    assert!(errors.iter().any(|error| error.contains("apple_password")));
    assert!(errors.iter().any(|error| error.contains("apple_api_key")));
    assert!(errors.iter().any(|error| error.contains("store_id")));
    assert!(errors.iter().any(|error| error.contains("license_key")));
    assert!(errors.iter().any(|error| error.contains("product id:")));
}

#[test]
fn rejects_apple_notarization_secret_assignments() {
    let text = "APPLE_API_ISSUER=uuid APPLE_ID=dev@example.com APPLE_TEAM_ID=ABCDE12345";

    let errors = violations("evidence", text);

    assert!(errors
        .iter()
        .any(|error| error.contains("apple_api_issuer")));
    assert!(errors.iter().any(|error| error.contains("apple_id")));
    assert!(errors.iter().any(|error| error.contains("apple_team_id")));
}

#[test]
fn rejects_apple_notarization_colon_secrets() {
    let text = "Apple API key: ABCDEF1234 APPLE_PASSWORD: app-pass Apple team id: ABCDE12345";

    let errors = violations("evidence", text);

    assert!(errors.iter().any(|error| error.contains("apple api key:")));
    assert!(errors.iter().any(|error| error.contains("apple_password:")));
    assert!(errors.iter().any(|error| error.contains("apple team id:")));
}

#[test]
fn rejects_human_readable_store_api_secrets() {
    let text = "Lemon Squeezy API key: live-secret";

    let errors = violations("evidence", text);

    assert!(errors
        .iter()
        .any(|error| error.contains("lemon squeezy api key:")));
}

#[test]
fn rejects_colon_form_store_and_variant_ids() {
    let text = "store id: 123; variant id: 456; LEMON_SQUEEZY_PRODUCT_ID: 789";

    let errors = violations("evidence", text);

    assert!(errors.iter().any(|error| error.contains("store id:")));
    assert!(errors.iter().any(|error| error.contains("variant id:")));
    assert!(errors
        .iter()
        .any(|error| error.contains("lemon_squeezy_product_id:")));
}

#[test]
fn rejects_lemon_squeezy_colon_ids() {
    let text = "LEMON_SQUEEZY_STORE_ID: 123 LEMON_SQUEEZY_VARIANT_ID: 456";

    let errors = violations("evidence", text);

    assert!(errors
        .iter()
        .any(|error| error.contains("lemon_squeezy_store_id:")));
    assert!(errors
        .iter()
        .any(|error| error.contains("lemon_squeezy_variant_id:")));
}

#[test]
fn rejects_secret_like_values_with_spaced_separators() {
    let text =
        "product_id : 123; store_id : 456; variant_id: 789; license_key: raw; raw_key : test";

    let errors = violations("evidence", text);

    assert!(errors.iter().any(|error| error.contains("product_id:")));
    assert!(errors.iter().any(|error| error.contains("store_id:")));
    assert!(errors.iter().any(|error| error.contains("variant_id:")));
    assert!(errors.iter().any(|error| error.contains("license_key:")));
    assert!(errors.iter().any(|error| error.contains("raw_key:")));
}

#[test]
fn rejects_license_key_with_spaced_equals() {
    let errors = violations("evidence", "license key = raw-test-key");

    assert!(errors.iter().any(|error| error.contains("license key=")));
}

#[test]
fn rejects_raw_license_key_forms() {
    let text = "raw license key: test-key raw_license_key = test-key";

    let errors = violations("evidence", text);

    assert!(errors
        .iter()
        .any(|error| error.contains("raw license key:")));
    assert!(errors
        .iter()
        .any(|error| error.contains("raw_license_key=")));
}

#[test]
fn allows_safe_evidence_terms() {
    let text =
        "license-key fingerprint exists, private store IDs absent, raw key absent from cache";

    assert!(violations("evidence", text).is_empty());
}
