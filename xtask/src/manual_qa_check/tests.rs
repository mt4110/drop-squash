use super::check_file;
use super::requirements::{REQUIRED_CHECKS, REQUIRED_FIELDS};

#[test]
fn accepts_complete_manual_qa_tables() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::create_dir(&artifact).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, complete_manual_qa(&artifact)).unwrap();

    assert!(check_file(&path).unwrap().is_empty());
}

#[test]
fn manual_qa_template_contains_required_labels() {
    let text = std::fs::read_to_string("../docs/manual-qa.md").unwrap();
    let labels = template_labels(&text);
    let mut missing = Vec::new();

    super::requirements::require_labels(
        "manual QA field is missing",
        &REQUIRED_FIELDS,
        &labels,
        &mut missing,
    );
    super::requirements::require_labels(
        "manual QA check is missing",
        &REQUIRED_CHECKS,
        &labels,
        &mut missing,
    );

    assert!(missing.is_empty(), "{missing:?}");
}

#[test]
fn reports_empty_environment_fields() {
    let (_directory, path) = write_manual_qa("| App build |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA field is empty: App build".to_string()));
}

#[test]
fn reports_app_build_without_commit_identity() {
    let (_directory, path) = write_manual_qa("| App build | 0.1.0 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("version and git commit")));
}

#[test]
fn reports_app_build_without_numeric_version() {
    let (_directory, path) = write_manual_qa("| App build | DropSquash 0.x.0 git abc1234 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("version and git commit")));
}

#[test]
fn reports_weak_environment_field_values() {
    let (_directory, path) = write_manual_qa(
        "| macOS version | Concrete evidence |\n\
| Machine | MacBookPro18,4 |\n\
| Output folder | /missing/dropsquash-output |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("macOS version")));
    assert!(missing
        .iter()
        .any(|error| error.contains("CPU architecture")));
    assert!(missing.iter().any(|error| error.contains("Output folder")));
}

#[test]
fn reports_weak_input_sample_set() {
    let (_directory, path) = write_manual_qa("| Input sample set | local files |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("short, medium, and large")));
}

#[test]
fn reports_wrong_state_paths() {
    let (_directory, path) = write_manual_qa(
        "| Config path | /tmp/config.json |\n\
| History path | /tmp/history.jsonl |\n\
| License cache path | /tmp/license.json |\n",
    );
    let missing = check_file(&path).unwrap();

    assert_eq!(
        missing
            .iter()
            .filter(|error| error.contains("DropSquash/"))
            .count(),
        3
    );
}

#[test]
fn accepts_inline_code_state_paths() {
    let (_directory, path) = write_manual_qa(
        "| Config path | `$HOME/Library/Application Support/DropSquash/config.json` |\n\
| History path | `$HOME/Library/Application Support/DropSquash/history.jsonl` |\n\
| License cache path | `$HOME/Library/Application Support/DropSquash/license.json` |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("DropSquash/")));
}

#[test]
fn reports_generic_tester_field() {
    let (_directory, path) = write_manual_qa("| Tester | Concrete evidence |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Tester")));
}

#[test]
fn reports_empty_four_column_results() {
    let (_directory, path) = write_manual_qa("| Convert | sample.mov | Smaller output |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result is empty: Convert".to_string()));
}

#[test]
fn reports_empty_three_column_results() {
    let (_directory, path) = write_manual_qa("| Release gate | Passes |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result is empty: Release gate".to_string()));
}

#[test]
fn reports_rows_with_unexpected_column_count() {
    let (_directory, path) =
        write_manual_qa("| Release gate | Passes | evidence before | evidence after | extra |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("unexpected column count: Release gate")));
}

#[test]
fn reports_missing_required_checks() {
    let (_directory, path) = write_manual_qa("| App build | 0.1.0 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA check is missing: Cancellation".to_string()));
    assert!(missing.contains(&"manual QA check is missing: Reveal privacy receipt".to_string()));
    assert!(missing.contains(&"manual QA check is missing: Queued job cancellation".to_string()));
    assert!(missing.contains(&"manual QA check is missing: Batch summary".to_string()));
}

#[test]
fn reports_missing_app_artifact_path() {
    let (_directory, path) = write_manual_qa("| App artifact | /missing/DropSquash.app |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("artifact does not exist")));
}

#[test]
fn reports_non_app_or_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.zip");
    std::fs::write(&artifact, "artifact").unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains(".app or .dmg")));
}

#[test]
fn reports_app_artifact_that_is_not_directory() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.app");
    std::fs::write(&artifact, "not a bundle").unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains(".app artifact")));
}

#[test]
fn reports_dmg_artifact_that_is_not_file() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::create_dir(&artifact).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains(".dmg artifact")));
}

#[test]
fn reports_dmg_artifact_without_udif_trailer() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, "not really a dmg").unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("UDIF DMG")));
}

#[test]
fn reports_non_iso_date() {
    let (_directory, path) = write_manual_qa("| Date | 7/11/2026 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("YYYY-MM-DD")));
}

#[test]
fn reports_out_of_range_date() {
    let (_directory, path) = write_manual_qa("| Date | 2026-99-99 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("YYYY-MM-DD")));
}

#[test]
fn reports_impossible_calendar_date() {
    let (_directory, path) = write_manual_qa("| Date | 2026-02-31 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("YYYY-MM-DD")));
}

#[test]
fn reports_unusable_historical_date() {
    let (_directory, path) = write_manual_qa("| Date | 0000-01-01 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("YYYY-MM-DD")));
}

#[test]
fn accepts_leap_day() {
    let (_directory, path) = write_manual_qa("| Date | 2028-02-29 |\n");
    let missing = check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("manual QA Date")));
}

#[test]
fn reports_vague_manual_results() {
    let (_directory, path) =
        write_manual_qa("| Cancellation | large.mov | Returns to ready | Pass |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Cancellation".to_string()));
}

#[test]
fn reports_generic_manual_results() {
    let (_directory, path) = write_manual_qa(
        "| Cancellation | large.mov | Returns to ready | Observed expected behavior |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Cancellation".to_string()));
}

#[test]
fn reports_placeholder_field_values() {
    let (_directory, path) = write_manual_qa("| App build | TBD |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA field needs evidence: App build".to_string()));
}

#[test]
fn reports_placeholder_four_column_results() {
    let (_directory, path) =
        write_manual_qa("| Cancellation | large.mov | Returns to ready | N/A |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Cancellation".to_string()));
}

#[test]
fn reports_placeholder_three_column_results() {
    let (_directory, path) =
        write_manual_qa("| Gatekeeper open test | Opens cleanly | Blocked |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Gatekeeper open test".to_string()));
}

#[test]
fn reports_incomplete_benchmark_results() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | local samples recorded |\n\
| Benchmark regression threshold | Throughput does not regress by more than 20% | no regression |\n\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>` | CSV recorded | Pass |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("machine, and OS")));
    assert!(missing.iter().any(|error| error.contains("20%")));
    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_incomplete_license_sandbox_results() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox product setup | Product exists | product ready |\n\
| Sandbox purchase | Checkout completes | order completed |\n\
| Empty key activation | Friendly validation error | empty message |\n\
| Invalid key activation | Friendly license error; no raw key persisted | error shown |\n\
| Valid sandbox activation | Pro state; raw key absent from cache | activated |\n\
| License network failure | Friendly network error; existing valid cache remains intact | network failed |\n\
| Forget license on this Mac | Local cache clears; app returns to trial or locked state | forgotten |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox product setup")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox purchase")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Empty key activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Invalid key activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Forget license on this Mac")));
}

#[test]
fn reports_forget_license_without_cache_removal() {
    let (_directory, path) = write_manual_qa(
        "| Forget license on this Mac | Local cache clears | license cache checked and app returned to trial state |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("cache removal")));
}

#[test]
fn reports_network_failure_without_existing_valid_cache() {
    let (_directory, path) = write_manual_qa(
        "| License network failure | Friendly network error | friendly network error shown and license.json cache preserved with no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn reports_activation_with_persisted_raw_key() {
    let (_directory, path) = write_manual_qa(
        "| Empty key activation | Friendly validation error | friendly validation shown and license.json cache checked with raw key persisted |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Empty key activation")));
}

#[test]
fn reports_incomplete_packaged_app_results() {
    let (_directory, path) = write_manual_qa(
        "| Choose recording conversion | Small `.mov` | Creates output | converted file |\n\
| Cancellation | Large recording | App returns ready | stopped |\n\
| Reveal output | Completed output link | Finder opens | opened |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Choose recording conversion")));
    assert!(missing.iter().any(|error| error.contains("Cancellation")));
    assert!(missing.iter().any(|error| error.contains("Reveal output")));
}

#[test]
fn reports_privacy_receipt_without_zero_upload_policy() {
    let (_directory, path) = write_manual_qa(
        "| Privacy receipt sidecar | Successful conversion | Creates receipt | clip.privacy.json recorded uploaded_bytes and metadata_policy |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Privacy receipt sidecar")));
}

#[test]
fn reports_reveal_results_without_selection_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Reveal privacy receipt | Successful conversion | Finder opens | Finder opened clip.privacy.json |\n\
| Reveal output | Completed output link | Finder opens | Finder opened clip.squashed.mp4 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Reveal privacy receipt")));
    assert!(missing.iter().any(|error| error.contains("Reveal output")));
}

#[test]
fn reports_duplicate_output_without_numbered_file_name() {
    let (_directory, path) = write_manual_qa(
        "| Duplicate output naming | Same recording twice | Numbered suffix | second output used numbered suffix |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Duplicate output naming")));
}

#[test]
fn reports_incomplete_release_candidate_results() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line recorded | checksum created |\n\
| Codesign verification | Developer ID signature | signature ok |\n\
| Gatekeeper open test | Signed app opens cleanly | opened |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("checksum")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Codesign verification")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper open test")));
}

#[test]
fn reports_checksum_result_without_digest() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg` | SHA-256 line recorded | SHA-256 line recorded for DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn reports_command_result_without_command_evidence() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- release-check` | Passes | Passes |\n\
| `cargo run -p xtask -- website-check` | Passes | website-check passed |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("release-check")));
    assert!(!missing.iter().any(|error| error.contains("website-check")));
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}

fn template_labels(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|') && !line.contains("---"))
        .filter_map(|line| {
            super::cells(line)
                .first()
                .map(|label| label.trim().to_string())
        })
        .filter(|label| label != "Field" && label != "Check")
        .collect()
}

fn complete_manual_qa(artifact: &std::path::Path) -> String {
    let output = artifact.parent().unwrap().join("qa-output");
    std::fs::create_dir(&output).unwrap();
    let mut text = String::from("| Field | Value |\n|---|---|\n");
    for field in REQUIRED_FIELDS {
        let value = match field {
            "App artifact" => artifact.display().to_string(),
            "App build" => "DropSquash 0.1.0 git abc1234".to_string(),
            "macOS version" => "macOS 26.5.2".to_string(),
            "Machine" => "MacBookPro18,4 arm64".to_string(),
            "Input sample set" => "short, medium, and large local recordings".to_string(),
            "Output folder" => output.display().to_string(),
            "Config path" => "$HOME/Library/Application Support/DropSquash/config.json".to_string(),
            "History path" => {
                "$HOME/Library/Application Support/DropSquash/history.jsonl".to_string()
            }
            "License cache path" => {
                "$HOME/Library/Application Support/DropSquash/license.json".to_string()
            }
            "Tester" => "masaki".to_string(),
            "Date" => "2026-07-11".to_string(),
            _ => "Concrete evidence".to_string(),
        };
        text.push_str(&format!("| {field} | {value} |\n"));
    }
    text.push_str("| Check | Expected | Result |\n|---|---|---|\n");
    for check in REQUIRED_CHECKS {
        if check.starts_with('`') {
            text.push_str(&command_result(check));
        } else if check == "Benchmark sample set" {
            text.push_str("| Benchmark sample set | Passes | short, medium, and large samples recorded on MacBookPro18,4 macOS 26.5.2 |\n");
        } else if check == "Benchmark regression threshold" {
            text.push_str(
                "| Benchmark regression threshold | Passes | no sample exceeded 20% regression |\n",
            );
        } else if check == "Sandbox product setup" {
            text.push_str("| Sandbox product setup | Passes | DropSquash intended product confirmed and license keys enabled |\n");
        } else if check == "Sandbox purchase" {
            text.push_str("| Sandbox purchase | Passes | intended product checkout completed by test buyer order abc123 |\n");
        } else if check == "Empty key activation" {
            text.push_str("| Empty key activation | Passes | friendly validation shown and license.json cache has no raw key |\n");
        } else if check == "Invalid key activation" {
            text.push_str("| Invalid key activation | Passes | friendly error shown and license.json cache has no raw key |\n");
        } else if check == "Valid sandbox activation" {
            text.push_str("| Valid sandbox activation | Passes | Pro state reached and raw key absent from license.json cache |\n");
        } else if check == "License network failure" {
            text.push_str("| License network failure | Passes | friendly network error shown and existing valid license.json cache preserved with no raw key |\n");
        } else if check == "Forget license on this Mac" {
            text.push_str("| Forget license on this Mac | Passes | license cache cleared and app returned to trial state |\n");
        } else if check == "Choose recording conversion" {
            text.push_str("| Choose recording conversion | Passes | saved clip.squashed.mp4 and original remained in place |\n");
        } else if check == "Drag-and-drop conversion" {
            text.push_str("| Drag-and-drop conversion | Passes | saved drag.squashed.mp4 and original remained in place |\n");
        } else if check == "Privacy receipt sidecar" {
            text.push_str("| Privacy receipt sidecar | Passes | clip.privacy.json recorded uploaded_bytes = 0 and metadata_policy = preserve |\n");
        } else if check == "Reveal privacy receipt" {
            text.push_str("| Reveal privacy receipt | Passes | Finder opened with clip.privacy.json selected |\n");
        } else if check == "Duplicate output naming" {
            text.push_str("| Duplicate output naming | Passes | second output used numbered clip.squashed-2.mp4 suffix |\n");
        } else if check == "Cancellation" {
            text.push_str("| Cancellation | Passes | app returned ready and trial history showed no new success |\n");
        } else if check == "Multi-file queue" {
            text.push_str("| Multi-file queue | Passes | three recordings queued with one active sequential conversion |\n");
        } else if check == "Queued job cancellation" {
            text.push_str("| Queued job cancellation | Passes | queued row marked cancelled and never started |\n");
        } else if check == "Batch summary" {
            text.push_str(
                "| Batch summary | Passes | summary showed finished count and saved bytes |\n",
            );
        } else if check == "Ask source policy" {
            text.push_str(
                "| Ask source policy | Passes | Ask prompt let tester choose Trash or Keep |\n",
            );
        } else if check == "Trash source policy" {
            text.push_str("| Trash source policy | Passes | original moved to Trash only after verified smaller output |\n");
        } else if check == "Failed conversion" {
            text.push_str("| Failed conversion | Passes | original remained and trial count unchanged after failure |\n");
        } else if check == "Larger output" {
            text.push_str(
                "| Larger output | Passes | larger result failed and trial count unchanged |\n",
            );
        } else if check == "Reveal output" {
            text.push_str(
                "| Reveal output | Passes | Finder opened with clip.squashed.mp4 selected |\n",
            );
        } else if check == "Codesign verification" {
            text.push_str("| Codesign verification | Passes | codesign verified Developer ID Application signature for DropSquash.dmg |\n");
        } else if check == "Notarization staple verification" {
            text.push_str("| Notarization staple verification | Passes | notary accepted and staple/spctl assessment passed for DropSquash.dmg |\n");
        } else if check == "Gatekeeper open test" {
            text.push_str("| Gatekeeper open test | Passes | Gatekeeper opened app cleanly in fresh macOS account |\n");
        } else {
            text.push_str(&format!(
                "| {check} | Passes | Evidence recorded with artifact, file name, or count |\n"
            ));
        }
    }
    text
}

fn command_result(check: &str) -> String {
    let result = match check {
        "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`" => {
            "artifact-check passed for DropSquash.dmg"
        }
        "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`" => {
            "SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef DropSquash.dmg"
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            "macos-signing-check passed in release environment"
        }
        "`cargo run -p xtask -- release-check`" => "release-check passed",
        "`cargo run -p xtask -- file-size-check`" => "file-size-check passed",
        "`cargo run -p xtask -- media-policy-check`" => "media-policy-check passed",
        "`cargo run -p xtask -- privacy-policy-check`" => "privacy-policy-check passed",
        "`cargo run -p xtask -- website-check`" => "website-check passed",
        "`cargo run -p xtask -- manual-qa-check`" => "manual-qa-check passed",
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp>`" => {
            "CSV recorded for three samples, outputs were smaller, saved outside repo at /tmp/dropsquash-bench/results.csv"
        }
        _ => "Pass",
    };
    format!("| {check} | Passes | {result} |\n")
}
