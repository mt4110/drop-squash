use super::{mapping, missing_manual_verified_evidence, PACKAGED_MACOS_EVIDENCE};

#[test]
fn accepts_verified_manual_blocker_with_manual_result() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox purchase | Checkout completes | Sandbox checkout completed for intended product with test buyer order abc123 |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_manual_blocker_without_manual_result() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache |  |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_manual_blocker_with_placeholder_result() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | TBD |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_manual_blocker_with_vague_result() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox purchase | Checkout completes | Pass |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_manual_blocker_with_malformed_manual_row() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox purchase | Checkout completes | extra | Completed for intended product with test buyer order abc123 | unexpected |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_manual_blocker_with_two_cell_check_row() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Sandbox purchase | Completed for intended product with test buyer order abc123 |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_manual_blocker_with_duplicate_manual_rows() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| Sandbox purchase | Checkout completes | Completed for intended product with test buyer order abc123 |
| Sandbox purchase | Checkout completes | Completed for intended product with test buyer order abc123 |
";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_sandbox_purchase_without_order() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox purchase | Checkout completes | Sandbox checkout completed for intended product with test buyer |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_sandbox_purchase_without_sandbox_context() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Sandbox purchase | Checkout completes | Completed for intended product with test buyer order abc123 |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_verified_product_setup_without_license_keys() {
    let blockers = "| Lemon Squeezy product setup | Verified | Sandbox product configured | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox product setup | DropSquash sandbox product exists | intended product confirmed |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy product setup"));
}

#[test]
fn accepts_verified_product_setup_with_license_keys() {
    let blockers = "| Lemon Squeezy product setup | Verified | Sandbox product configured | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox product setup | DropSquash sandbox product exists | sandbox product is the intended product, DropSquash; license keys enabled |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_product_setup_without_sandbox_context() {
    let blockers = "| Lemon Squeezy product setup | Verified | Sandbox product configured | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox product setup | DropSquash product exists | DropSquash intended product confirmed; license keys enabled |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Lemon Squeezy product setup"));
}

#[test]
fn reports_verified_invalid_key_blocker_without_manual_result() {
    let blockers = "| Invalid license key handling | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Invalid key activation | Friendly license error; no raw key persisted |  |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Invalid license key handling"));
}

#[test]
fn reports_verified_empty_key_blocker_without_cache_evidence() {
    let blockers = "| Empty key activation | Verified | Friendly validation error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Empty key activation | Friendly validation error | friendly validation shown |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Empty key activation"));
}

#[test]
fn accepts_verified_empty_key_blocker_with_cache_evidence() {
    let blockers = "| Empty key activation | Verified | Friendly validation error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Empty key activation | Empty key leaves Activate disabled | Activate disabled for empty input and license.json cache has no raw key, no fingerprint, and no instance |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_empty_key_without_fingerprint_instance_absence() {
    let blockers = "| Empty key activation | Verified | Friendly validation error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Empty key activation | Empty key leaves Activate disabled | Activate disabled for empty input and license.json cache has no raw key |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Empty key activation"));
}

#[test]
fn reports_verified_network_failure_without_preserved_cache() {
    let blockers = "| License network failure | Verified | Friendly network error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| License network failure | Friendly network error | friendly network error shown |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"License network failure"));
}

#[test]
fn reports_verified_network_failure_without_existing_valid_cache() {
    let blockers = "| License network failure | Verified | Friendly network error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| License network failure | Friendly network error | friendly network error shown and license.json cache preserved with no raw key |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"License network failure"));
}

#[test]
fn accepts_verified_network_failure_with_preserved_cache() {
    let blockers = "| License network failure | Verified | Friendly network error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance fields with no raw key |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_network_failure_without_hex_fingerprint_evidence() {
    let blockers = "| License network failure | Verified | Friendly network error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved fingerprint and instance fields with no raw key |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"License network failure"));
}

#[test]
fn reports_verified_network_failure_without_fingerprint_instance_evidence() {
    let blockers = "| License network failure | Verified | Friendly network error | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved with no raw key |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"License network failure"));
}

#[test]
fn reports_verified_local_forget_blocker_without_manual_result() {
    let blockers = "| Local license forget | Verified | Local cache removed | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Forget license on this Mac | Local cache clears; app returns to trial or locked state | TBD |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Local license forget"));
}

#[test]
fn reports_verified_local_forget_without_cache_removal() {
    let blockers = "| Local license forget | Verified | Local cache removed | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Forget license on this Mac | Local cache clears; app returns to trial or locked state | license cache checked and app returned to trial state |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Local license forget"));
}

#[test]
fn reports_verified_license_blocker_without_cache_evidence() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Pro reached and raw key absent |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_license_blocker_with_persisted_raw_key() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Pro reached and license.json cache checked with raw key persisted |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_license_blocker_without_sandbox_context() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Activating state disabled submit; Pro reached; license.json cache checked; raw key absent |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_license_blocker_without_fingerprint_instance_evidence() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, Pro reached; license.json cache checked; raw key absent |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_verified_license_blocker_without_hex_fingerprint_evidence() {
    let blockers = "| Valid sandbox activation | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Valid sandbox activation | Pro state; raw key absent from cache | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, Pro reached; license.json cache checked fingerprint and instance fields; raw key absent |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Valid sandbox activation"));
}

#[test]
fn accepts_verified_license_blocker_with_cache_evidence() {
    let blockers = "| Invalid license key handling | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Invalid key activation | Friendly license error; no raw key persisted | Activating state disabled submit; friendly error shown; license.json cache checked; raw key absent, no fingerprint, and no instance |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_invalid_key_without_fingerprint_instance_absence() {
    let blockers = "| Invalid license key handling | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Invalid key activation | Friendly license error; no raw key persisted | Activating state disabled submit; friendly error shown; license.json cache checked; raw key absent |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Invalid license key handling"));
}

#[test]
fn reports_incomplete_packaged_macos_manual_qa() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Choose recording conversion | Small recording | Creates smaller MP4 | Concrete file output |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_without_specific_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Choose recording conversion",
        "conversion finished with output",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_environment_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("App build", "0.1.0");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_commit_matching_prefix() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "App build",
        &format!("DropSquash 0.1.0 git {}ffff", current_head()),
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_malformed_metadata_row() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("App build", "DropSquash 0.1.0 git abc1234").replace(
        "| App build | DropSquash 0.1.0 git abc1234 |",
        "| App build | Expected | DropSquash 0.1.0 git abc1234 |",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_noncanonical_artifact() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("App artifact", "/tmp/Other.app");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_vague_build_identity() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("App build", "DropSquash git build");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_old_build_head() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("App build", "DropSquash 0.1.0 git 0000000");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_non_iso_date() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("Date", "July 11 2026");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_invalid_calendar_date() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("Date", "2026-99-99");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_placeholder_state_path() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Config path",
        "$HOME/Library/Application Support/DropSquash/config.json",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_without_manual_check_result() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("`cargo run -p xtask -- manual-qa-check`", "not run yet");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_privacy_receipt() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Privacy receipt sidecar",
        "clip.privacy.json recorded uploaded_bytes and metadata_policy",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_without_smaller_output_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Choose recording conversion",
        "saved clip.squashed.mp4 and original remained in place",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_reveal_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("Reveal output", "Finder opened with clip.squashed.mp4");

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_batch_summary() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Batch summary",
        "summary showed finished count, saved bytes, and mixed outcomes",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_word_only_queue_counts() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Multi-file queue",
        "three recordings queued with one active sequential conversion; completed job finished and unrelated failures did not block it",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_word_only_batch_counts() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Batch summary",
        "summary showed finished count, saved bytes, failed count, cancelled count, and blocked count",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_unverified_trash_output() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Trash source policy",
        "button showed Moving original and was disabled; original moved to Trash after smaller output",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_failed_conversion() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Failed conversion",
        "original remained and trial count unchanged after failure",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_macos_manual_qa_with_weak_duplicate_name() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Duplicate output naming",
        "second output used numbered suffix",
    );

    let missing = missing_manual_verified_evidence(blockers, &manual);

    assert!(missing.contains(&"Packaged macOS manual QA"));
}

#[test]
fn accepts_packaged_macos_manual_qa_with_specific_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with(
        "Choose recording conversion",
        "saved smaller clip.squashed.mp4 and original remained in place",
    );

    assert!(missing_manual_verified_evidence(blockers, &manual).is_empty());
}

#[test]
fn ignores_blocked_manual_blockers() {
    let blockers = "| Gatekeeper clean-machine open | Blocked | Fresh macOS account opens app | TBD | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly |  |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_gatekeeper_blocker_without_clean_machine_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | app opened |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn reports_verified_gatekeeper_blocker_without_signed_notarized_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened app cleanly in fresh macOS account |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn accepts_verified_gatekeeper_blocker_with_clean_machine_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh macOS account without Gatekeeper warning |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_gatekeeper_blocker_without_public_dmg_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed, notarized, stapled app cleanly in fresh macOS account without Gatekeeper warning |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn reports_verified_gatekeeper_blocker_without_staple_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed and notarized app cleanly in fresh macOS account without Gatekeeper warning |\n";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn reports_verified_benchmark_blocker_without_threshold() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples produced smaller outputs on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv |
| Benchmark regression threshold | Passes | no regression recorded |
";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Benchmark release set"));
}

#[test]
fn reports_verified_benchmark_blocker_without_smaller_outputs() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples recorded on MacBookPro18,4 macOS 26.5.2 |
| Benchmark regression threshold | Passes | no sample exceeded 20% regression |
";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Benchmark release set"));
}

#[test]
fn reports_verified_benchmark_blocker_without_csv_path() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples produced smaller outputs on MacBookPro18,4 macOS 26.5.2 |
| Benchmark regression threshold | Passes | no sample exceeded 20% regression |
";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Benchmark release set"));
}

#[test]
fn accepts_verified_benchmark_blocker_with_release_set_evidence() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo at /tmp/dropsquash-bench/results.csv |
| Benchmark sample set | Passes | short, medium, and large samples produced smaller outputs on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv |
| Benchmark regression threshold | Passes | no sample exceeded 20% regression |
";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn accepts_verified_benchmark_blocker_with_percent_wording() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo at /tmp/dropsquash-bench/results.csv |
| Benchmark sample set | Passes | short, medium, and large samples produced smaller outputs on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv |
| Benchmark regression threshold | Passes | no sample exceeded 20 percent regression |
";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

fn packaged_manual_qa_with(check: &str, result: &str) -> String {
    PACKAGED_MACOS_EVIDENCE
        .iter()
        .map(|label| {
            let result = if *label == check {
                result.to_string()
            } else {
                packaged_result(label)
            };
            if mapping::is_metadata_field(label) {
                format!("| {label} | {result} |\n")
            } else {
                format!("| {label} | Expected | {result} |\n")
            }
        })
        .collect()
}

fn packaged_result(label: &str) -> String {
    match label {
        "App build" => format!("DropSquash 0.1.0 git {}", current_head()),
        "App artifact" => "/tmp/DropSquash.app".into(),
        "macOS version" => "macOS 15.5".into(),
        "Machine" => "Apple silicon Mac arm64".into(),
        "Input sample set" => "short, medium, and large local recordings".into(),
        "Output folder" => "/tmp/dropsquash-manual-qa-output".into(),
        "Config path" => "/Users/me/Library/Application Support/DropSquash/config.json".into(),
        "History path" => {
            "/Users/me/Library/Application Support/DropSquash/history.jsonl".into()
        }
        "License cache path" => {
            "/Users/me/Library/Application Support/DropSquash/license.json".into()
        }
        "Tester" => "Manual tester".into(),
        "Date" => "2026-07-11".into(),
        "Choose recording conversion" => {
            "saved smaller clip.squashed.mp4 and original remained in place".into()
        }
        "Drag-and-drop conversion" => {
            "saved smaller drag.squashed.mp4 and original remained in place".into()
        }
        "Privacy receipt sidecar" => {
            "clip.privacy.json recorded uploaded_bytes = 0, metadata_policy = preserve, file names instead of absolute paths".into()
        }
        "Reveal privacy receipt" => "Finder opened with clip.privacy.json selected".into(),
        "Duplicate output naming" => "second output used numbered clip.squashed-2.mp4 suffix".into(),
        "Cancellation" => "app returned ready and trial history showed no new success".into(),
        "Multi-file queue" => {
            "3 recordings queued with 1 active sequential conversion; completed job finished and unrelated failures did not block it".into()
        }
        "Queued job cancellation" => {
            "queued row marked cancelled and never started; trial history showed no new success".into()
        }
        "Batch summary" => {
            "summary showed finished count 2, saved bytes 123456, failed 0, cancelled 1, and blocked 0".into()
        }
        "Ask source policy" => "Ask prompt let tester choose Trash or Keep; original remained unchanged".into(),
        "Trash source policy" => {
            "button showed Moving original and was disabled; original moved to Trash only after verified smaller output".into()
        }
        "Failed conversion" => {
            "friendly error appeared; original remained and trial count unchanged after failure".into()
        }
        "Larger output" => "larger result failed; original remained and trial count unchanged".into(),
        "Reveal output" => "Finder opened with clip.squashed.mp4 selected".into(),
        "`cargo run -p xtask -- manual-qa-check`" => "manual-qa-check passed".into(),
        _ => "concrete evidence".into(),
    }
}

fn current_head() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
