use super::response::LicenseInstance;
use super::*;

#[test]
fn activation_uses_instance_id_and_fingerprint_only() {
    let activation = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(true),
            valid: None,
            deactivated: None,
            error: None,
            instance: Some(LicenseInstance {
                id: "instance-1".to_string(),
            }),
        },
    )
    .unwrap();

    assert_eq!(activation.instance_id, "instance-1");
    assert_ne!(activation.license_key_fingerprint, "LS-SECRET-RAW-KEY");
    assert!(activation.valid);
}

#[test]
fn activation_error_is_friendly() {
    let error = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(false),
            valid: None,
            deactivated: None,
            error: Some("Activation limit reached.".to_string()),
            instance: Some(LicenseInstance {
                id: "instance-1".to_string(),
            }),
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("Activation limit reached."));
    assert!(!error.to_string().contains("LS-SECRET-RAW-KEY"));
}

#[test]
fn activation_error_redacts_echoed_license_key() {
    let error = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(false),
            valid: None,
            deactivated: None,
            error: Some("Key LS-SECRET-RAW-KEY is not valid.".to_string()),
            instance: None,
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("[license key]"));
    assert!(!error.to_string().contains("LS-SECRET-RAW-KEY"));
}
