use super::check_file;
use super::requirements::{REQUIRED_CHECKS, REQUIRED_FIELDS};

#[test]
fn accepts_complete_manual_qa_tables() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, complete_manual_qa(&artifact)).unwrap();

    let missing = check_file(&path).unwrap();
    assert!(missing.is_empty(), "{missing:?}");
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
fn manual_qa_template_has_no_untracked_labels() {
    let text = std::fs::read_to_string("../docs/manual-qa.md").unwrap();
    let labels = template_labels(&text);
    let allowed = REQUIRED_FIELDS
        .iter()
        .chain(REQUIRED_CHECKS.iter())
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let untracked = labels
        .iter()
        .filter(|label| !allowed.contains(label.as_str()))
        .collect::<Vec<_>>();

    assert!(untracked.is_empty(), "{untracked:?}");
}

#[test]
fn reports_empty_environment_fields() {
    let (_directory, path) = write_manual_qa("| App build |  |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA field is empty: App build".to_string()));
}

#[test]
fn rejects_prepared_manual_qa_draft_marker() {
    let (_directory, path) = write_manual_qa(
        "Prepared manual QA draft only. Replace this file with concrete observations.\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("prepared draft markers")));
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
fn reports_app_build_commit_with_matching_prefix() {
    let head = current_head();
    let (_directory, path) = write_manual_qa(&format!(
        "| App build | DropSquash 0.1.0 git {head}ffff |\n"
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("current HEAD")));
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
fn reports_non_local_input_sample_set() {
    let (_directory, path) =
        write_manual_qa("| Input sample set | short, medium, and large recordings |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("local")));
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
fn accepts_absolute_state_paths() {
    let (_directory, path) = write_manual_qa(
        "| Config path | /Users/me/Library/Application Support/DropSquash/config.json |\n\
| History path | /Users/me/Library/Application Support/DropSquash/history.jsonl |\n\
| License cache path | /Users/me/Library/Application Support/DropSquash/license.json |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(!missing.iter().any(|error| error.contains("DropSquash/")));
}

#[test]
fn reports_home_placeholder_state_paths() {
    let (_directory, path) = write_manual_qa(
        "| Config path | `$HOME/Library/Application Support/DropSquash/config.json` |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("absolute DropSquash")));
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
fn reports_duplicate_required_field_labels() {
    let head = current_head();
    let (_directory, path) = write_manual_qa(&format!(
        "| App build | DropSquash 0.1.0 git {head} |\n\
| App build | DropSquash 0.1.0 git {head} |\n"
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA label is duplicated: App build".to_string()));
}

#[test]
fn reports_app_build_for_old_head() {
    let (_directory, path) = write_manual_qa("| App build | DropSquash 0.1.0 git 0000000 |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("current HEAD")));
}

#[test]
fn reports_duplicate_required_check_labels() {
    let (_directory, path) = write_manual_qa(
        "| Reveal output | Completed output link | Finder opens | Finder opened clip.squashed.mp4 selected |\n\
| Reveal output | Completed output link | Finder opens | Finder opened clip.squashed.mp4 selected |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA label is duplicated: Reveal output".to_string()));
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
fn reports_noncanonical_app_artifact_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.app");
    std::fs::create_dir(&artifact).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("DropSquash.app")));
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
fn reports_noncanonical_dmg_artifact_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!("| App artifact | {} |\n", artifact.display()),
    )
    .unwrap();

    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("DropSquash.dmg")));
}

#[test]
fn reports_release_command_results_for_different_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for public UDIF DropSquash.dmg |\n\
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef DropSquash.dmg |\n",
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("artifact-check") && error.contains("Other.dmg")));
    assert!(missing
        .iter()
        .any(|error| error.contains("checksum") && error.contains("Other.dmg")));
}

#[test]
fn reports_gatekeeper_result_without_dmg_artifact_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed, notarized, stapled app cleanly in fresh macOS account without Gatekeeper warning |\n",
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper open test") && error.contains("DropSquash.dmg")));
}

#[test]
fn reports_checksum_result_for_different_dmg_digest() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef DropSquash.dmg |\n",
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("checksum") && error.contains("digest")));
}

#[test]
fn reports_release_command_results_without_artifact_path() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let bytes = dmg_bytes(b"dropsquash");
    std::fs::write(&artifact, &bytes).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for public UDIF DropSquash.dmg |\n\
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 {} DropSquash.dmg |\n",
            artifact.display(),
            sha256_hex(&bytes)
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("artifact-check") && error.contains("path")));
    assert!(missing
        .iter()
        .any(|error| error.contains("checksum") && error.contains("path")));
}

#[test]
fn accepts_checksum_result_matching_dmg_digest() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let bytes = dmg_bytes(b"dropsquash");
    std::fs::write(&artifact, &bytes).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 {} DropSquash.dmg |\n",
            artifact.display(),
            sha256_hex(&bytes)
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(!missing
        .iter()
        .any(|error| error.contains("checksum") && error.contains("digest")));
}

#[test]
fn accepts_release_command_results_with_artifact_path() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let bytes = dmg_bytes(b"dropsquash");
    std::fs::write(&artifact, &bytes).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for public UDIF {} |\n\
| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 {} {} |\n",
            artifact.display(),
            artifact.display(),
            sha256_hex(&bytes),
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(!missing
        .iter()
        .any(|error| error.contains("App artifact path")));
}

#[test]
fn reports_artifact_check_without_public_udif_context() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check passed for {} |\n",
            artifact.display(),
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("public UDIF DropSquash.dmg")));
}

#[test]
fn reports_signing_results_for_different_dmg_artifact() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("Other.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| Codesign verification | Passes | codesign verified Developer ID Application signature for public DropSquash.dmg |\n\
| Notarization staple verification | Passes | notary accepted and staple/spctl assessment passed for public DropSquash.dmg |\n",
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Codesign") && error.contains("Other.dmg")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization") && error.contains("Other.dmg")));
}

#[test]
fn reports_signing_results_without_artifact_path() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(
        &path,
        format!(
            "| App artifact | {} |\n\
| Codesign verification | Passes | codesign verified Developer ID Application signature for public DropSquash.dmg |\n\
| Notarization staple verification | Passes | notary accepted, stapler validate passed, and spctl accepted for public DropSquash.dmg |\n\
| Gatekeeper open test | Passes | Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh macOS account without Gatekeeper warning |\n",
            artifact.display()
        ),
    )
    .unwrap();
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Codesign") && error.contains("path")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization") && error.contains("path")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper") && error.contains("path")));
}

#[test]
fn reports_notarization_without_stapled_status() {
    let (_directory, path) = write_manual_qa(
        "| Notarization staple verification | Passes | notary accepted, stapler ran, and spctl accepted for public DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization staple verification")));
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
fn reports_short_success_manual_results() {
    let (_directory, path) =
        write_manual_qa("| Cancellation | large.mov | Returns to ready | Success |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Cancellation".to_string()));
}

#[test]
fn reports_looks_good_manual_results() {
    let (_directory, path) =
        write_manual_qa("| Gatekeeper open test | Opens cleanly | Looks good |\n");
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Gatekeeper open test".to_string()));
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
fn reports_embedded_placeholder_results() {
    let (_directory, path) = write_manual_qa(
        "| Gatekeeper open test | Opens cleanly | Signed app opens cleanly; TBD after notarization |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.contains(&"manual QA result needs evidence: Gatekeeper open test".to_string()));
}

#[test]
fn reports_incomplete_benchmark_results() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | local samples recorded |\n\
| Benchmark regression threshold | Throughput does not regress by more than 20% | no regression |\n\
| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | Pass |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
    assert!(missing.iter().any(|error| error.contains("baseline")));
    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_benchmark_threshold_without_release_candidate_baseline() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark regression threshold | Throughput does not regress by more than 20% | no sample exceeded 20% regression against the same-machine baseline |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("release candidate baseline")));
}

#[test]
fn reports_benchmark_command_without_csv_path() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | CSV recorded for three samples, outputs were smaller, saved outside repo |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_benchmark_command_with_repo_local_csv_path() {
    let repo_csv = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("dropsquash-bench.csv");
    let (_directory, path) = write_manual_qa(&format!(
        "| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | CSV recorded for three samples, outputs were smaller, saved outside repo at {} |\n",
        repo_csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_benchmark_command_with_normalized_repo_local_csv_path() {
    let repo_csv = normalized_repo_path("target/dropsquash-bench.csv");
    let (_directory, path) = write_manual_qa(&format!(
        "| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | CSV recorded for three samples, outputs were smaller, saved outside repo at {} |\n",
        repo_csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_benchmark_command_with_missing_csv_file() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("missing.csv");
    let (_manual_directory, path) = write_manual_qa(&format!(
        "| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | CSV recorded for three samples, outputs were smaller, saved outside repo at {} |\n",
        csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("benchmark")));
}

#[test]
fn reports_benchmark_sample_set_without_smaller_outputs() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples on MacBookPro18,4 macOS 26.5 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("smaller outputs")));
}

#[test]
fn reports_benchmark_sample_set_without_csv_path() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn reports_benchmark_sample_set_without_three_sample_context() {
    let directory = tempfile::tempdir().unwrap();
    let csv = csv_file(directory.path(), "results.csv");
    let (_manual_directory, path) = write_manual_qa(&format!(
        "| Benchmark sample set | Short, medium, and large samples | short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with csv={} |\n",
        csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("three samples")));
}

#[test]
fn reports_benchmark_sample_set_with_relative_csv_path() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with CSV saved outside repo at results.csv |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn reports_benchmark_sample_set_with_repo_local_csv_path() {
    let repo_csv = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("dropsquash-bench.csv");
    let (_directory, path) = write_manual_qa(&format!(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with CSV saved outside repo at {} |\n",
        repo_csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn accepts_benchmark_sample_set_with_labeled_csv_path() {
    let directory = tempfile::tempdir().unwrap();
    let csv = csv_file(directory.path(), "results.csv");
    let (_manual_directory, path) = write_manual_qa(&format!(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with csv={} |\n",
        csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(!missing
        .iter()
        .any(|error| error.contains("Benchmark sample set")));
}

#[test]
fn reports_mismatched_benchmark_command_and_sample_set_csv_paths() {
    let directory = tempfile::tempdir().unwrap();
    let command_csv = csv_file(directory.path(), "command.csv");
    let sample_csv = csv_file(directory.path(), "sample.csv");
    let (_manual_directory, path) = write_manual_qa(&format!(
        "| `cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>` | CSV recorded | CSV recorded for three samples, outputs were smaller, saved outside repo at {} |\n\
| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with csv={} |\n",
        command_csv.display(),
        sample_csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("same CSV path")));
}

#[test]
fn reports_benchmark_sample_set_with_missing_csv_file() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("missing.csv");
    let (_manual_directory, path) = write_manual_qa(&format!(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5 with csv={} |\n",
        csv.display()
    ));
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("CSV path outside repo")));
}

#[test]
fn reports_benchmark_sample_set_without_backend() {
    let (_directory, path) = write_manual_qa(
        "| Benchmark sample set | Short, medium, and large samples | three short medium large samples produced smaller outputs on MacBookPro18,4 macOS 26.5 with CSV saved outside repo at /tmp/dropsquash-bench/results.csv |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("backend")));
}

#[test]
fn reports_incomplete_license_sandbox_results() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox product setup | Product exists | product ready |\n\
| Sandbox purchase | Checkout completes | order completed |\n\
| Empty key activation | Empty key leaves Activate disabled | empty message |\n\
| Invalid key activation | Friendly license error; no raw key persisted | error shown |\n\
| Valid sandbox activation | Pro state; raw key absent from cache | activated |\n\
| License network failure | Friendly network error; existing valid cache remains intact | network failed |\n\
| Expired license refresh | Reconnect prompt | expired cache |\n\
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
        .any(|error| error.contains("Expired license refresh")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Forget license on this Mac")));
}

#[test]
fn reports_valid_activation_without_sandbox_context() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Activating state disabled submit; Pro state reached and license.json cache has no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_product_setup_without_sandbox_context() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox product setup | Product exists | DropSquash intended product confirmed and license keys enabled |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox product setup")));
}

#[test]
fn reports_product_setup_without_private_store_id_absence() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox product setup | Product exists | DropSquash sandbox intended product confirmed and license keys enabled |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox product setup")));
}

#[test]
fn reports_purchase_without_sandbox_context() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox purchase | Checkout completes | intended product checkout completed by test buyer order abc123 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox purchase")));
}

#[test]
fn reports_purchase_without_checkout_context() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox purchase | Checkout completes | sandbox completed for intended product by test buyer order abc123 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox purchase")));
}

#[test]
fn reports_purchase_without_concrete_order_id() {
    let (_directory, path) = write_manual_qa(
        "| Sandbox purchase | Checkout completes | sandbox checkout completed for intended product by test buyer order completed |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Sandbox purchase")));
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
fn reports_forget_license_without_observed_cache_and_state() {
    let (_directory, path) = write_manual_qa(
        "| Forget license on this Mac | Local cache clears | Forgetting state disabled action; license cache cleared and app returned to trial state |\n",
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
fn reports_empty_key_without_cache_inspection() {
    let (_directory, path) = write_manual_qa(
        "| Empty key activation | Empty key leaves Activate disabled | Activate disabled for empty input and license.json cache has no raw key, no fingerprint, and no instance |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Empty key activation")));
}

#[test]
fn reports_invalid_key_without_cache_inspection() {
    let (_directory, path) = write_manual_qa(
        "| Invalid key activation | Friendly license error | Activating state disabled submit; friendly error shown and license.json cache has no raw key, no fingerprint, and no instance |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Invalid key activation")));
}

#[test]
fn reports_valid_activation_without_fingerprint_instance_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and raw key absent from license.json cache |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_valid_activation_without_cache_observation() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and license.json cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_network_failure_without_fingerprint_instance_evidence() {
    let (_directory, path) = write_manual_qa(
        "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved with no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn reports_valid_activation_without_hex_fingerprint_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and license.json cache preserved fingerprint and instance fields with raw key absent |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_valid_activation_without_instance_id_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and license.json cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance fields with raw key absent |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_network_failure_without_hex_fingerprint_evidence() {
    let (_directory, path) = write_manual_qa(
        "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved fingerprint and instance fields with no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn reports_network_failure_without_instance_id_evidence() {
    let (_directory, path) = write_manual_qa(
        "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance fields with no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn reports_network_failure_without_cache_observation() {
    let (_directory, path) = write_manual_qa(
        "| License network failure | Friendly network error | friendly network error shown and existing valid license.json cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("License network failure")));
}

#[test]
fn reports_expired_refresh_without_reconnect_prompt() {
    let (_directory, path) = write_manual_qa(
        "| Expired license refresh | Reconnect prompt | expired offline grace cache blocked conversion and license.json cache has no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Expired license refresh")));
}

#[test]
fn reports_expired_refresh_without_pre_start_block() {
    let (_directory, path) = write_manual_qa(
        "| Expired license refresh | Reconnect prompt | expired offline grace license.json cache showed reconnect prompt, blocked conversion, and had no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Expired license refresh")));
}

#[test]
fn reports_expired_refresh_without_conversion_attempt() {
    let (_directory, path) = write_manual_qa(
        "| Expired license refresh | Reconnect prompt | expired offline grace license.json cache showed reconnect prompt, blocked conversion before starting, and had no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Expired license refresh")));
}

#[test]
fn reports_expired_refresh_without_cache_observation() {
    let (_directory, path) = write_manual_qa(
        "| Expired license refresh | Reconnect prompt | attempted conversion with expired offline grace license.json cache showed reconnect prompt, blocked conversion before starting, and had no raw key |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Expired license refresh")));
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
fn reports_activation_with_contradictory_raw_key_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and checked license.json cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent but raw key persisted |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
}

#[test]
fn reports_license_results_without_action_state() {
    let (_directory, path) = write_manual_qa(
        "| Invalid key activation | Friendly license error | friendly error shown and license.json cache has no raw key |\n\
| Valid sandbox activation | Pro state | Pro state reached and license.json cache has no raw key |\n\
| Forget license on this Mac | Local cache clears | license cache cleared and app returned to trial state |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Invalid key activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Forget license on this Mac")));
}

#[test]
fn reports_activating_state_without_submit_context() {
    let (_directory, path) = write_manual_qa(
        "| Invalid key activation | Friendly license error | Activating state disabled; friendly error shown and inspected license.json cache has no raw key, no fingerprint, and no instance |\n\
| Valid sandbox activation | Pro state | Lemon Squeezy sandbox activation request entered Activating state, disabled, reached Pro state, and license.json cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Invalid key activation")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Valid sandbox activation")));
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
fn reports_conversion_without_smaller_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Choose recording conversion | Small `.mov` | Creates output | saved clip.squashed.mp4 and original remained in place |\n\
| Drag-and-drop conversion | Small `.mov` | Creates output | saved drag.squashed.mp4 and original remained in place |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Choose recording conversion")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Drag-and-drop conversion")));
}

#[test]
fn reports_conversion_without_original_remained_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Choose recording conversion | Small `.mov` | Creates output | saved smaller clip.squashed.mp4 and original checked |\n\
| Drag-and-drop conversion | Small `.mov` | Creates output | saved smaller drag.squashed.mp4 and original checked |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Choose recording conversion")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Drag-and-drop conversion")));
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
fn reports_privacy_receipt_without_file_name_policy() {
    let (_directory, path) = write_manual_qa(
        "| Privacy receipt sidecar | Successful conversion | Creates receipt | clip.privacy.json recorded uploaded_bytes = 0 and metadata_policy = preserve |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Privacy receipt sidecar")));
}

#[test]
fn reports_privacy_receipt_with_absolute_paths() {
    let (_directory, path) = write_manual_qa(
        "| Privacy receipt sidecar | Successful conversion | Creates receipt | clip.privacy.json recorded uploaded_bytes = 0, metadata_policy = preserve, file names, and absolute paths |\n",
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
fn reports_duplicate_output_without_second_output_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Duplicate output naming | Same recording twice | Numbered suffix | numbered clip.squashed-2.mp4 suffix appeared |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Duplicate output naming")));
}

#[test]
fn reports_reveal_output_without_squashed_file_name() {
    let (_directory, path) = write_manual_qa(
        "| Reveal output | Completed output link | Finder opens | Finder opened with clip.mp4 selected |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Reveal output")));
}

#[test]
fn reports_trash_source_without_progress_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Trash source policy | Successful conversion | Original moves | original moved to Trash after verified smaller output |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Trash source policy")));
}

#[test]
fn reports_trash_source_without_original_move_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Trash source policy | Successful conversion | Original moves | button showed Moving and was disabled after verified smaller output |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Trash source policy")));
}

#[test]
fn reports_trash_source_without_smaller_verification() {
    let (_directory, path) = write_manual_qa(
        "| Trash source policy | Successful conversion | Original moves | button showed Moving and was disabled; original moved to Trash only after verified output |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Trash source policy")));
}

#[test]
fn reports_trash_source_without_only_after_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Trash source policy | Successful conversion | Original moves | button showed Moving original and was disabled; original moved to Trash after verified smaller output |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Trash source policy")));
}

#[test]
fn reports_failed_conversion_without_friendly_error() {
    let (_directory, path) = write_manual_qa(
        "| Failed conversion | Unsupported input | Original remains | original remained and trial count unchanged after failure |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Failed conversion")));
}

#[test]
fn reports_failed_conversion_without_original_remained_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Failed conversion | Unsupported input | Original remains | friendly error shown; original checked and trial count unchanged after failure |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Failed conversion")));
}

#[test]
fn reports_larger_output_without_original_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Larger output | Input that cannot be made smaller | Treated as failure | failed and trial count unchanged |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Larger output")));
}

#[test]
fn reports_larger_output_without_original_remained_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Larger output | Input that cannot be made smaller | Treated as failure | larger not smaller result failed; original checked and trial count unchanged |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Larger output")));
}

#[test]
fn reports_larger_output_without_larger_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Larger output | Input that cannot be made smaller | Treated as failure | result failed, original remained, and trial count unchanged |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Larger output")));
}

#[test]
fn reports_cancellation_without_success_history_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Cancellation | Large recording | App returns ready | app returned ready and trial history was checked |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Cancellation")));
}

#[test]
fn reports_cancellation_without_trial_count_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Cancellation | Large recording | App returns ready | app returned ready and trial history showed no new success |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Cancellation")));
}

#[test]
fn reports_cancellation_without_temp_cleanup_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Cancellation | Large recording | App returns ready | app returned ready, trial count unchanged, and history showed no new success |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Cancellation")));
}

#[test]
fn reports_queued_cancellation_without_success_history_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Queued job cancellation | Three recordings | Waiting row cancelled | queued row marked cancelled and never started |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Queued job cancellation")));
}

#[test]
fn reports_queued_cancellation_without_trial_count_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Queued job cancellation | Three recordings | Waiting row cancelled | queued row marked cancelled and never started; trial history showed no new success |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Queued job cancellation")));
}

#[test]
fn reports_queued_cancellation_without_waiting_row_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Queued job cancellation | Three recordings | Waiting row cancelled | cancellation marked cancelled and never started; trial count unchanged and history showed no new success |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Queued job cancellation")));
}

#[test]
fn reports_multi_file_queue_without_numeric_queue_counts() {
    let (_directory, path) = write_manual_qa(
        "| Multi-file queue | Three recordings | Queue runs sequentially | three recordings queued with one active sequential conversion; unrelated failure did not block finished jobs |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Multi-file queue")));
}

#[test]
fn reports_multi_file_queue_without_failure_unblock_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Multi-file queue | Three recordings | Queue runs sequentially | 3 recordings queued with 1 active sequential conversion and 3 jobs finished |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Multi-file queue")));
}

#[test]
fn reports_batch_summary_without_numeric_counts() {
    let (_directory, path) = write_manual_qa(
        "| Batch summary | Three recordings | Queue summary | summary showed finished count, saved bytes, and cancelled mixed outcome |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Batch summary")));
}

#[test]
fn reports_batch_summary_without_all_mixed_counts() {
    let (_directory, path) = write_manual_qa(
        "| Batch summary | Three recordings | Queue summary | summary showed finished count 2, saved bytes 123456, and cancelled 1 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Batch summary")));
}

#[test]
fn reports_batch_summary_without_lock_blocking_context() {
    let (_directory, path) = write_manual_qa(
        "| Batch summary | Three recordings | Queue summary | summary showed finished count 2, saved bytes 123456, failed 0, cancelled 1, and blocked 0 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("Batch summary")));
}

#[test]
fn reports_ask_source_without_original_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Ask source policy | Successful conversion | User can choose | Ask prompt let tester choose Trash or Keep |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Ask source policy")));
}

#[test]
fn reports_incomplete_release_candidate_results() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | checksum created |\n\
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
fn reports_release_candidate_results_without_public_context() {
    let (_directory, path) = write_manual_qa(
        "| Codesign verification | Developer ID signature | codesign verified Developer ID Application signature for DropSquash.dmg |\n\
| Notarization staple verification | Notary assessment | notary accepted, stapler validate passed, and spctl accepted for DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Codesign verification")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization staple verification")));
}

#[test]
fn reports_release_candidate_results_without_dmg_context() {
    let (_directory, path) = write_manual_qa(
        "| Codesign verification | Developer ID signature | codesign verified Developer ID Application signature for public artifact |\n\
| Notarization staple verification | Notary assessment | notary accepted, stapler validate passed, and spctl accepted for public artifact |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Codesign verification")));
    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization staple verification")));
}

#[test]
fn reports_notarization_result_without_spctl_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Notarization staple verification | Notary assessment | notary accepted and stapler validate passed for public DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization staple verification")));
}

#[test]
fn reports_notarization_result_without_stapler_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Notarization staple verification | Notary assessment | notary accepted and spctl accepted for public DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Notarization staple verification")));
}

#[test]
fn reports_gatekeeper_result_without_warning_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed, notarized, stapled app cleanly in fresh macOS account |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper open test")));
}

#[test]
fn reports_gatekeeper_result_without_staple_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed and notarized app cleanly in fresh macOS account without Gatekeeper warning |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper open test")));
}

#[test]
fn reports_gatekeeper_result_without_public_dmg_evidence() {
    let (_directory, path) = write_manual_qa(
        "| Gatekeeper open test | Signed app opens cleanly | Gatekeeper opened signed, notarized, stapled app cleanly in fresh macOS account without Gatekeeper warning |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("Gatekeeper open test")));
}

#[test]
fn reports_checksum_result_without_digest() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 line recorded for DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn reports_checksum_result_with_uppercase_digest() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789 DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn reports_checksum_result_with_placeholder_digest() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA256SUMS created with SHA-256 0000000000000000000000000000000000000000000000000000000000000000 DropSquash.dmg |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("checksum")));
}

#[test]
fn reports_secret_like_manual_qa_values() {
    let (_directory, path) = write_manual_qa(
        "| Apple notary log | Accepted | accepted with APPLE_PASSWORD=not-for-release |\n\
| Valid sandbox activation | Pro state | raw license key: test-key was entered and variant_id=123 |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("secret-like value")));
}

#[test]
fn reports_checksum_result_without_sha256sums_output() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS` | SHA-256 line recorded | SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef DropSquash.dmg |\n",
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

#[test]
fn reports_release_artifact_commands_without_pass_evidence() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- artifact-check path/to/DropSquash.dmg` | Passes | artifact-check DropSquash.dmg |\n\
| `cargo run -p xtask -- macos-signing-check` | Passes | macos-signing-check release environment |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing.iter().any(|error| error.contains("artifact-check")));
    assert!(missing
        .iter()
        .any(|error| error.contains("macos-signing-check")));
}

#[test]
fn reports_homebrew_cask_check_without_paths() {
    let (_directory, path) = write_manual_qa(
        "| `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Passes | homebrew-cask-check passed |\n",
    );
    let missing = check_file(&path).unwrap();

    assert!(missing
        .iter()
        .any(|error| error.contains("homebrew-cask-check")));
}

fn write_manual_qa(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("manual-qa.md");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}

fn csv_file(directory: &std::path::Path, name: &str) -> std::path::PathBuf {
    let path = directory.join(name);
    std::fs::write(&path, "sample,duration_ms\nshort,100\n").unwrap();
    path
}

fn template_labels(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|') && !line.contains("---"))
        .filter_map(|line| {
            super::rows::cells(line)
                .first()
                .map(|label| label.trim().to_string())
        })
        .filter(|label| label != "Field" && label != "Check")
        .collect()
}

fn complete_manual_qa(artifact: &std::path::Path) -> String {
    let output = artifact.parent().unwrap().join("qa-output");
    let benchmark_csv = csv_file(artifact.parent().unwrap(), "benchmark-results.csv");
    std::fs::create_dir(&output).unwrap();
    let mut text = String::from("| Field | Value |\n|---|---|\n");
    for field in REQUIRED_FIELDS {
        let value = match field {
            "App artifact" => artifact.display().to_string(),
            "App build" => format!("DropSquash 0.1.0 git {}", current_head()),
            "macOS version" => "macOS 26.5.2".to_string(),
            "Machine" => "MacBookPro18,4 arm64".to_string(),
            "Input sample set" => "short, medium, and large local recordings".to_string(),
            "Output folder" => output.display().to_string(),
            "Config path" => {
                "/Users/me/Library/Application Support/DropSquash/config.json".to_string()
            }
            "History path" => {
                "/Users/me/Library/Application Support/DropSquash/history.jsonl".to_string()
            }
            "License cache path" => {
                "/Users/me/Library/Application Support/DropSquash/license.json".to_string()
            }
            "Tester" => "masaki".to_string(),
            "Date" => "2026-07-11".to_string(),
            _ => "Concrete evidence".to_string(),
        };
        text.push_str(&format!("| {field} | {value} |\n"));
    }
    text.push_str("| Check | Expected | Result |\n|---|---|---|\n");
    for check in REQUIRED_CHECKS {
        if check
            == "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`"
        {
            text.push_str(&format!("| {check} | Passes | CSV recorded for three samples, outputs were smaller, saved outside repo at {} |\n", benchmark_csv.display()));
        } else if check.starts_with('`') {
            text.push_str(&command_result(check, artifact));
        } else if check == "Benchmark sample set" {
            text.push_str(&format!("| Benchmark sample set | Passes | three short, medium, and large samples produced smaller outputs with backend apple-native, saved percent, duration, and speed ratio on MacBookPro18,4 macOS 26.5.2 with CSV saved outside repo at {} |\n", benchmark_csv.display()));
        } else if check == "Benchmark regression threshold" {
            text.push_str(
                "| Benchmark regression threshold | Passes | no sample exceeded 20% regression against the same-machine release candidate baseline |\n",
            );
        } else if check == "Sandbox product setup" {
            text.push_str("| Sandbox product setup | Passes | DropSquash sandbox intended product confirmed, license keys enabled, and private store IDs not recorded |\n");
        } else if check == "Sandbox purchase" {
            text.push_str("| Sandbox purchase | Passes | sandbox checkout completed for intended product by test buyer order abc123 |\n");
        } else if check == "Empty key activation" {
            text.push_str("| Empty key activation | Passes | Activate disabled for empty input and checked license.json cache has no raw key, no fingerprint, and no instance |\n");
        } else if check == "Invalid key activation" {
            text.push_str("| Invalid key activation | Passes | Activating state disabled submit; friendly error shown and inspected license.json cache has no raw key, no fingerprint, and no instance |\n");
        } else if check == "Valid sandbox activation" {
            text.push_str("| Valid sandbox activation | Passes | Lemon Squeezy sandbox activation request entered Activating state, disabled submit, reached Pro state, and checked license.json cache kept fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with raw key absent |\n");
        } else if check == "License network failure" {
            text.push_str("| License network failure | Passes | friendly network error shown and checked existing valid license.json cache preserved fingerprint 1111111111111111111111111111111111111111111111111111111111111111 plus instance_id field with no raw key |\n");
        } else if check == "Expired license refresh" {
            text.push_str("| Expired license refresh | Passes | attempted conversion with expired offline grace license.json cache; reconnect prompt appeared, blocked conversion before starting, and checked cache had no raw key |\n");
        } else if check == "Forget license on this Mac" {
            text.push_str("| Forget license on this Mac | Passes | Forgetting state disabled action; confirmed license cache cleared and observed app returned to trial state |\n");
        } else if check == "Choose recording conversion" {
            text.push_str("| Choose recording conversion | Passes | saved smaller clip.squashed.mp4 and original remained in place |\n");
        } else if check == "Drag-and-drop conversion" {
            text.push_str("| Drag-and-drop conversion | Passes | saved smaller drag.squashed.mp4 and original remained in place |\n");
        } else if check == "Privacy receipt sidecar" {
            text.push_str("| Privacy receipt sidecar | Passes | clip.privacy.json recorded uploaded_bytes = 0, metadata_policy = preserve, and file names instead of absolute paths |\n");
        } else if check == "Reveal privacy receipt" {
            text.push_str("| Reveal privacy receipt | Passes | Finder opened with clip.privacy.json selected |\n");
        } else if check == "Duplicate output naming" {
            text.push_str("| Duplicate output naming | Passes | second output used numbered clip.squashed-2.mp4 suffix |\n");
        } else if check == "Cancellation" {
            text.push_str("| Cancellation | Passes | app returned ready after temp cleanup; trial count unchanged; history showed no new success |\n");
        } else if check == "Multi-file queue" {
            text.push_str("| Multi-file queue | Passes | 3 recordings queued with 1 active sequential conversion; 3 jobs finished and unrelated failure did not block the queue |\n");
        } else if check == "Queued job cancellation" {
            text.push_str("| Queued job cancellation | Passes | queued row marked cancelled and never started; trial count unchanged and history showed no new success |\n");
        } else if check == "Batch summary" {
            text.push_str(
                "| Batch summary | Passes | trial lock blocked pending jobs; summary showed finished count 2, saved bytes 123456, failed 0, cancelled 1, blocked 0 |\n",
            );
        } else if check == "Ask source policy" {
            text.push_str(
                "| Ask source policy | Passes | Ask prompt let tester choose Trash or Keep while original remained unchanged |\n",
            );
        } else if check == "Trash source policy" {
            text.push_str("| Trash source policy | Passes | button showed Moving original and was disabled; original moved to Trash only after verified smaller output |\n");
        } else if check == "Failed conversion" {
            text.push_str("| Failed conversion | Passes | friendly error shown; original remained and trial count unchanged after failure |\n");
        } else if check == "Larger output" {
            text.push_str(
                "| Larger output | Passes | larger not smaller result failed, original remained, and trial count unchanged |\n",
            );
        } else if check == "Reveal output" {
            text.push_str(
                "| Reveal output | Passes | Finder opened with clip.squashed.mp4 selected |\n",
            );
        } else if check == "Codesign verification" {
            text.push_str(&format!(
                "| Codesign verification | Passes | codesign verified Developer ID Application signature for public {} |\n",
                artifact.display()
            ));
        } else if check == "Notarization staple verification" {
            text.push_str(&format!(
                "| Notarization staple verification | Passes | notary accepted, stapler validate passed, and spctl accepted for public {} |\n",
                artifact.display()
            ));
        } else if check == "Gatekeeper open test" {
            text.push_str(&format!(
                "| Gatekeeper open test | Passes | Gatekeeper opened signed, notarized, stapled app from public {} cleanly in fresh macOS account without Gatekeeper warning |\n",
                artifact.display()
            ));
        } else {
            text.push_str(&format!(
                "| {check} | Passes | Evidence recorded with artifact, file name, or count |\n"
            ));
        }
    }
    text
}

fn command_result(check: &str, artifact: &std::path::Path) -> String {
    let digest = std::fs::read(artifact)
        .map(|bytes| sha256_hex(&bytes))
        .unwrap_or_else(|_| {
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()
        });
    let result = match check {
        "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`" => {
            format!("artifact-check passed for public UDIF {}", artifact.display())
        }
        "`cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS`" => {
            format!("SHA256SUMS created with SHA-256 {digest} {}", artifact.display())
        }
        "`cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md`" => {
            "homebrew-cask-check passed for packaging/homebrew/Casks/dropsquash.rb and /tmp/dropsquash-release-notes.md".to_string()
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            "macos-signing-check passed in release environment".to_string()
        }
        "`cargo run -p xtask -- release-check`" => "release-check passed".to_string(),
        "`cargo run -p xtask -- file-size-check`" => "file-size-check passed".to_string(),
        "`cargo run -p xtask -- media-policy-check`" => "media-policy-check passed".to_string(),
        "`cargo run -p xtask -- privacy-policy-check`" => "privacy-policy-check passed".to_string(),
        "`cargo run -p xtask -- website-check`" => "website-check passed".to_string(),
        "`cargo run -p xtask -- manual-qa-check`" => "manual-qa-check passed".to_string(),
        "`cargo run -p xtask -- benchmark --release-set --input <short> --input <medium> --input <large> --output-dir <tmp> --csv-output <tmp/results.csv>`" => {
            "CSV recorded for three samples, outputs were smaller, saved outside repo at /tmp/dropsquash-bench/results.csv".to_string()
        }
        _ => "Pass".to_string(),
    };
    format!("| {check} | Passes | {result} |\n")
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = [0_u8; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend_from_slice(&trailer);
    bytes
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn current_head() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn normalized_repo_path(child: &str) -> std::path::PathBuf {
    let cwd = std::env::current_dir().unwrap();
    cwd.parent()
        .unwrap()
        .join("outside")
        .join("..")
        .join(cwd.file_name().unwrap())
        .join(child)
}
