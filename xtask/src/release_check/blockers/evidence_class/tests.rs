use super::{unclassified_blockers, unknown_classification_rows, ALLOWED_CLASSES};
use crate::release_check::blockers::REQUIRED_BLOCKERS;

#[test]
fn accepts_complete_evidence_classes() {
    let text = REQUIRED_BLOCKERS
        .iter()
        .enumerate()
        .map(|(index, blocker)| {
            let class = ALLOWED_CLASSES[index % ALLOWED_CLASSES.len()];
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
    let text = "| Valid sandbox activation | License sandbox | Activate the packaged app and inspect the local cache | Release notes |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_signing_classification_without_release_notes_owner() {
    let text = "| Signed DMG | Signing/notarization | Sign the public DropSquash.dmg and capture Developer ID verification output | `docs/manual-qa.md` |\n";

    let unclassified = unclassified_blockers(text);

    assert!(unclassified.contains(&"Signed DMG"));
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
