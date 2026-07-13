use super::{classes, unclassified_blockers, unknown_classification_rows};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_complete_evidence_classes() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            let class = classes::expected(blocker).unwrap();
            let action = action_for(blocker);
            let owner = owner_for(blocker);
            format!("| {blocker} | {class} | {action} | {owner} |\n")
        })
        .collect::<String>();

    assert!(unclassified_blockers(&text).is_empty());
}

#[test]
fn release_blockers_template_classifies_required_rows() {
    let text = std::fs::read_to_string("../docs/release-blockers.md").unwrap();

    assert!(unclassified_blockers(&text).is_empty());
}

#[test]
fn reports_missing_evidence_classification() {
    let unclassified = unclassified_blockers("");

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_placeholder_next_action() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | License sandbox | Capture concrete release evidence | TBD |\n")
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_embedded_placeholder_classification() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | Distribution | Capture concrete release evidence TBD | TODO owner |\n"
            )
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Published checksum"));
}

#[test]
fn reports_unknown_evidence_class() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!(
                "| {blocker} | External | Capture concrete release evidence | Record the evidence in the named location |\n"
            )
        })
        .collect::<String>();

    let unclassified = unclassified_blockers(&text);

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_mismatched_evidence_class() {
    let text = "| Valid sandbox activation | Distribution | Activate the packaged app and inspect the local cache | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_unknown_evidence_classification_rows() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .map(|blocker| {
            format!("| {blocker} | Distribution | Capture concrete release evidence | Release owner |\n")
        })
        .chain(std::iter::once(
            "| Extra launch task | Distribution | Capture concrete release evidence | Release owner |\n"
                .to_string(),
        ))
        .collect::<String>();

    let unknown = unknown_classification_rows(&text);

    assert_eq!(unknown, vec!["Extra launch task"]);
}

#[test]
fn ignores_evidence_classification_header_rows() {
    let unknown = unknown_classification_rows(
        "| Blocker | Class | Next action | Evidence owner |\n|---|---|---|---|\n",
    );

    assert!(unknown.is_empty());
}

#[test]
fn reports_packaged_manual_action_without_canonical_artifacts() {
    let text = "| Packaged macOS manual QA | Manual packaged-app | Run the packaged artifact through manual QA | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_manual_action_without_public_dmg() {
    let text = "| Packaged macOS manual QA | Manual packaged-app | Run the packaged DropSquash.dmg artifact through manual QA | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_public_url_classification_without_matching_owner_field() {
    let text = "| Published checksum | Distribution | Attach SHA256SUMS containing the public DropSquash.dmg SHA-256 line to the GitHub Release | Release owner |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Published checksum"));
}

#[test]
fn reports_manual_classification_without_manual_qa_owner() {
    let text = "| Valid sandbox activation | License sandbox | Run the Lemon Squeezy sandbox activation request, confirm submit is disabled while Activating, and inspect the local license cache | Release notes |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_license_action_without_sandbox_activation_request() {
    let text = "| Valid sandbox activation | License sandbox | Activate the packaged app, confirm submit is disabled while Activating, and inspect the local license cache | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_license_action_without_cache_identity_detail() {
    let text = "| Valid sandbox activation | License sandbox | Run the Lemon Squeezy sandbox activation request, confirm submit is disabled while Activating, and inspect the local license cache | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_license_action_without_cache_detail() {
    let text = "| License network failure | License sandbox | Simulate a failed activation request and inspect the friendly error | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"License network failure"));
}

#[test]
fn reports_network_failure_action_without_cache_identity_detail() {
    let text = "| License network failure | License sandbox | Simulate a failed activation request and inspect the friendly error plus preserved local cache | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"License network failure"));
}

#[test]
fn reports_public_website_action_without_required_pages() {
    let text = "| Public website deployment | Public web | Deploy the production site and verify public pages | Public website URL |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Public website deployment"));
}

#[test]
fn reports_live_checkout_action_without_product_context() {
    let text = "| Live checkout link | Public web | Verify the pricing page opens checkout | Live checkout URL |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Live checkout link"));
}

#[test]
fn reports_signing_classification_without_release_notes_owner() {
    let text = "| Signed DMG | Signing/notarization | Sign the public DropSquash.dmg and capture `codesign` Developer ID verification output | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_signing_action_without_codesign_detail() {
    let text = "| Signed DMG | Signing/notarization | Sign the public DropSquash.dmg and capture Developer ID verification output | Release notes |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Signed DMG"));
}

#[test]
fn reports_notarization_action_without_spctl_or_stapler_detail() {
    let text = "| Notarized and stapled DMG | Signing/notarization | Notarize and assess the public DropSquash.dmg with captured verification output | Release notes |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Notarized and stapled DMG"));
}

#[test]
fn reports_benchmark_action_without_external_csv_path() {
    let text = "| Benchmark release set | Benchmark | Run the release-set benchmark and record threshold evidence | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Benchmark release set"));
}

#[test]
fn reports_checksum_action_without_public_sha256_artifact() {
    let text = "| Published checksum | Distribution | Attach checksum file to the GitHub Release | GitHub Release URL |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Published checksum"));
}

#[test]
fn reports_homebrew_action_without_versioned_artifact() {
    let text = "| Homebrew cask install | Distribution | Open the Homebrew tap PR and verify the cask install command, DropSquash.dmg URL, matching SHA-256, auto_updates false, and zap cleanup path | Homebrew tap PR URL |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Homebrew cask install"));
}

#[test]
fn reports_homebrew_action_without_install_policy_details() {
    let text = "| Homebrew cask install | Distribution | Open tap PR and verify versioned DropSquash.dmg install evidence | Homebrew tap PR URL |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Homebrew cask install"));
}

fn action_for(blocker: &str) -> &'static str {
    match blocker {
        "Packaged macOS manual QA" => "Run public DropSquash.dmg artifact through manual QA",
        "Lemon Squeezy product setup" => {
            "Confirm sandbox product is the intended product, DropSquash, with license keys"
        }
        "Lemon Squeezy sandbox purchase" => {
            "Complete sandbox checkout for the intended product, test buyer, and order"
        }
        "Empty key activation" => {
            "Leave key empty, confirm Activate is disabled, and inspect local license cache raw-key, fingerprint, and instance absence"
        }
        "Valid sandbox activation" => {
            "Run the Lemon Squeezy sandbox activation request, confirm submit is disabled while Activating, and inspect local license cache 64-character hex fingerprint, `instance_id`, plus raw-key absence"
        }
        "Invalid license key handling" => {
            "Enter invalid key, confirm submit is disabled while activating, friendly error appears, and inspect local license cache raw-key, fingerprint, and instance absence"
        }
        "License network failure" => {
            "Simulate failed activation request and inspect friendly error plus preserved local cache 64-character hex fingerprint, `instance_id`, and raw-key absence"
        }
        "Local license forget" => {
            "Use local forget action, confirm disabled while forgetting, and inspect returned app state"
        }
        "Public website deployment" => {
            "Deploy the production site and verify release-status, privacy, pricing, support, and download pages"
        }
        "Refund policy finalized" => {
            "Publish the final refund policy URL before checkout goes live"
        }
        "Live checkout link" => {
            "Verify the public pricing page opens the tested Lemon Squeezy checkout for the intended product"
        }
        "Signed DMG" => {
            "Sign the public DropSquash.dmg and capture `codesign` Developer ID verification output"
        }
        "Notarized and stapled DMG" => {
            "Notarize, staple, and assess the public DropSquash.dmg with captured `spctl`, notary, and stapler verification output"
        }
        "Benchmark release set" => {
            "Run release-set benchmark and record absolute CSV path outside repo"
        }
        "Published checksum" => {
            "Attach SHA256SUMS containing public DropSquash.dmg SHA-256 line to the GitHub Release"
        }
        "Homebrew cask install" => {
            "Open Homebrew tap PR and verify brew install --cask for versioned DropSquash.dmg, matching SHA-256, auto_updates false, and zap cleanup"
        }
        _ => "Capture concrete release evidence",
    }
}

fn owner_for(blocker: &str) -> &'static str {
    if let Some((_, field)) = crate::release_url_fields::PAIRS
        .iter()
        .find(|(candidate, _)| *candidate == blocker)
    {
        return field;
    }
    match blocker {
        "Signed DMG" | "Notarized and stapled DMG" => "Release notes",
        _ => "`docs/manual-qa.md`",
    }
}
