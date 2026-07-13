use super::{plan, Request};

#[test]
fn plans_signing_steps_in_safe_order() {
    let request = Request {
        unsigned: "target/release/bundle/dmg/DropSquash.dmg".into(),
        output_dir: "/tmp/dropsquash-signed".into(),
    };

    let steps = plan(&request).unwrap();

    assert_eq!(steps.len(), 10);
    assert!(steps[0].contains("signed-dmg-prepare"));
    assert!(steps[1].contains("signed-dmg-copy"));
    assert!(steps[2].contains("macos-keychain-plan"));
    assert!(steps[3].contains("macos-codesign-plan"));
    assert!(steps[4].contains("macos-codesign-verify-plan"));
    assert!(steps[5].contains("macos-notary-plan"));
    assert!(steps[6].contains("macos-stapler-plan"));
    assert!(steps[7].contains("macos-spctl-plan"));
    assert!(steps[8].contains("signed-dmg-check"));
    assert!(steps[9].contains("macos-keychain-cleanup-plan"));
}

#[test]
fn plan_uses_canonical_signed_target() {
    let request = Request {
        unsigned: "/tmp/unsigned/DropSquash.dmg".into(),
        output_dir: "/tmp/signed".into(),
    };

    let steps = plan(&request).unwrap();

    assert!(steps
        .iter()
        .any(|step| step.contains("/tmp/signed/DropSquash.dmg")));
    assert!(steps
        .iter()
        .any(|step| step.contains("/tmp/signed/keychain")));
}

#[test]
fn rejects_output_that_would_overwrite_unsigned_input() {
    let request = Request {
        unsigned: "/tmp/dmg/DropSquash.dmg".into(),
        output_dir: "/tmp/dmg".into(),
    };

    let error = plan(&request).unwrap_err();

    assert!(error.contains("must not overwrite"));
}

#[test]
fn plan_does_not_print_secret_values() {
    let request = Request {
        unsigned: "/tmp/unsigned/DropSquash.dmg".into(),
        output_dir: "/tmp/signed".into(),
    };

    let text = plan(&request).unwrap().join("\n");

    assert!(!text.contains("APPLE_CERTIFICATE_PASSWORD"));
    assert!(!text.contains("APPLE_PASSWORD"));
    assert!(!text.contains("APPLE_API_KEY_P8"));
}
