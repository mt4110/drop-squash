use super::{missing_manual_verified_evidence, PACKAGED_MACOS_EVIDENCE};

#[test]
fn accepts_verified_manual_blocker_with_manual_result() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual =
        "| Sandbox purchase | Checkout completes | Completed for intended product with test buyer order abc123 |\n";

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
fn reports_verified_sandbox_purchase_without_order() {
    let blockers = "| Lemon Squeezy sandbox purchase | Verified | Sandbox checkout completes | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Sandbox purchase | Checkout completes | Completed for intended product with test buyer |\n";

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
    let manual = "| Sandbox product setup | DropSquash sandbox product exists | DropSquash intended product confirmed; license keys enabled |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
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
    let manual = "| Empty key activation | Friendly validation error | friendly validation shown and license.json cache has no raw key |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
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
    let manual = "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved with no raw key |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
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
fn accepts_verified_license_blocker_with_cache_evidence() {
    let blockers = "| Invalid license key handling | Verified | raw key is absent | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Invalid key activation | Friendly license error; no raw key persisted | friendly error shown; license.json cache checked; raw key absent |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
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
fn reports_packaged_macos_manual_qa_with_weak_reveal_evidence() {
    let blockers = "| Packaged macOS manual QA | Verified | Filled manual QA table | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = packaged_manual_qa_with("Reveal output", "Finder opened with clip.squashed.mp4");

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
        "saved clip.squashed.mp4 and original remained in place",
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
fn accepts_verified_gatekeeper_blocker_with_clean_machine_evidence() {
    let blockers = "| Gatekeeper clean-machine open | Verified | Fresh macOS account opens app | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened app cleanly in fresh macOS account |\n";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn reports_verified_benchmark_blocker_without_threshold() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples recorded on MacBookPro18,4 macOS 26.5.2 |
| Benchmark regression threshold | Passes | no regression recorded |
";

    let missing = missing_manual_verified_evidence(blockers, manual);

    assert!(missing.contains(&"Benchmark release set"));
}

#[test]
fn accepts_verified_benchmark_blocker_with_release_set_evidence() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples recorded on MacBookPro18,4 macOS 26.5.2 |
| Benchmark regression threshold | Passes | no sample exceeded 20% regression |
";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

#[test]
fn accepts_verified_benchmark_blocker_with_percent_wording() {
    let blockers = "| Benchmark release set | Verified | Release-set benchmark CSV covers samples | `docs/manual-qa.md` | `docs/manual-qa.md` |\n";
    let manual = "\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | CSV recorded | benchmark CSV recorded for three samples with smaller outputs outside repo |
| Benchmark sample set | Passes | short, medium, and large samples recorded on MacBookPro18,4 macOS 26.5.2 |
| Benchmark regression threshold | Passes | no sample exceeded 20 percent regression |
";

    assert!(missing_manual_verified_evidence(blockers, manual).is_empty());
}

fn packaged_manual_qa_with(check: &str, result: &str) -> String {
    PACKAGED_MACOS_EVIDENCE
        .iter()
        .map(|label| {
            let result = if *label == check {
                result
            } else {
                packaged_result(label)
            };
            format!("| {label} | Expected | {result} |\n")
        })
        .collect()
}

fn packaged_result(label: &str) -> &'static str {
    match label {
        "App build" => "0.1.0",
        "App artifact" => "/tmp/DropSquash.app",
        "macOS version" => "macOS 15.5",
        "Machine" => "Apple silicon Mac",
        "Input sample set" => "short, medium, and large local recordings",
        "Output folder" => "/tmp/dropsquash-manual-qa-output",
        "Tester" => "Manual tester",
        "Date" => "2026-07-11",
        "Choose recording conversion" => "saved clip.squashed.mp4 and original remained in place",
        "Drag-and-drop conversion" => "saved drag.squashed.mp4 and original remained in place",
        "Privacy receipt sidecar" => {
            "clip.privacy.json recorded uploaded_bytes = 0 and metadata_policy = preserve"
        }
        "Reveal privacy receipt" => "Finder opened with clip.privacy.json selected",
        "Duplicate output naming" => "second output used numbered clip.squashed-2.mp4 suffix",
        "Cancellation" => "app returned ready and trial history showed no new success",
        "Multi-file queue" => "three recordings queued with one active sequential conversion",
        "Queued job cancellation" => "queued row marked cancelled and never started",
        "Batch summary" => "summary showed finished count and saved bytes",
        "Ask source policy" => "Ask prompt let tester choose Trash or Keep",
        "Trash source policy" => "original moved to Trash only after verified smaller output",
        "Failed conversion" => "original remained and trial count unchanged after failure",
        "Larger output" => "larger result failed and trial count unchanged",
        "Reveal output" => "Finder opened with clip.squashed.mp4 selected",
        _ => "concrete evidence",
    }
}
