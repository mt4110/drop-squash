mod artifact_age;
mod artifact_check;
mod benchmark;
mod benchmark_csv_check;
mod checksum;
mod csv_evidence;
mod dmg;
mod dmg_cleanup_claims;
mod file_size_check;
mod git_head_match;
mod git_status;
mod github_release_plan;
mod homebrew_cask;
mod homebrew_cask_check;
mod macos_codesign_plan;
mod macos_codesign_verify_plan;
mod macos_keychain_cleanup_plan;
mod macos_keychain_plan;
mod macos_notary_plan;
mod macos_signing_check;
mod macos_signing_plan;
mod macos_spctl_plan;
mod macos_stapler_plan;
mod manual_qa_bad_input;
mod manual_qa_benchmark_fill;
mod manual_qa_check;
mod manual_qa_clean_draft;
mod manual_qa_fill_benchmark_threshold;
mod manual_qa_fill_check;
mod manual_qa_fill_local_proof;
mod manual_qa_installed_app;
mod manual_qa_link_samples;
mod manual_qa_merge_prepared;
mod manual_qa_pending;
mod manual_qa_prepare;
mod manual_qa_ready_distribution;
mod manual_qa_ready_license;
mod manual_qa_ready_all;
mod manual_qa_ready_local_proof;
mod manual_qa_release_gate_fill;
mod media_policy_check;
mod normalize_dmg;
mod privacy_policy_check;
mod productization_status;
mod public_url;
mod publish_check;
mod release_check;
mod release_notes_check;
mod release_notes_prepare;
mod release_url_fields;
mod secret_text;
mod signed_dmg_check;
mod signed_dmg_copy;
mod signed_dmg_prepare;
mod url_origin;
mod url_scheme;
mod website_check;

fn main() {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("artifact-check") => artifact_check::run(args.collect()),
        Some("benchmark") => benchmark::run(args.collect()),
        Some("benchmark-csv-check") => benchmark_csv_check::run(args.collect()),
        Some("checksum") => checksum::run(args.collect()),
        Some("file-size-check") => file_size_check::run(args.collect()),
        Some("github-release-plan") => github_release_plan::run(args.collect()),
        Some("homebrew-cask") => homebrew_cask::run(args.collect()),
        Some("homebrew-cask-check") => homebrew_cask_check::run(args.collect()),
        Some("macos-codesign-verify-plan") => macos_codesign_verify_plan::run(args.collect()),
        Some("macos-codesign-plan") => macos_codesign_plan::run(args.collect()),
        Some("macos-keychain-cleanup-plan") => macos_keychain_cleanup_plan::run(args.collect()),
        Some("macos-keychain-plan") => macos_keychain_plan::run(args.collect()),
        Some("macos-notary-plan") => macos_notary_plan::run(args.collect()),
        Some("macos-signing-plan") => macos_signing_plan::run(args.collect()),
        Some("macos-signing-check") => macos_signing_check::run(),
        Some("macos-spctl-plan") => macos_spctl_plan::run(args.collect()),
        Some("macos-stapler-plan") => macos_stapler_plan::run(args.collect()),
        Some("manual-qa-clean-draft") => manual_qa_clean_draft::run(args.collect()),
        Some("manual-qa-bad-input") => manual_qa_bad_input::run(args.collect()),
        Some("manual-qa-fill-benchmark") => manual_qa_benchmark_fill::run(args.collect()),
        Some("manual-qa-fill-benchmark-threshold") => {
            manual_qa_fill_benchmark_threshold::run(args.collect())
        }
        Some("manual-qa-fill-check") => manual_qa_fill_check::run(args.collect()),
        Some("manual-qa-fill-local-proof") => manual_qa_fill_local_proof::run(args.collect()),
        Some("manual-qa-check") => manual_qa_check::run(args.collect()),
        Some("manual-qa-installed-app") => manual_qa_installed_app::run(args.collect()),
        Some("manual-qa-link-samples") => manual_qa_link_samples::run(args.collect()),
        Some("manual-qa-merge-prepared") => manual_qa_merge_prepared::run(args.collect()),
        Some("manual-qa-pending") => manual_qa_pending::run(args.collect()),
        Some("manual-qa-fill-release-gates") => manual_qa_release_gate_fill::run(args.collect()),
        Some("manual-qa-prepare") => manual_qa_prepare::run(args.collect()),
        Some("manual-qa-ready-all") => manual_qa_ready_all::run(args.collect()),
        Some("manual-qa-ready-distribution") => manual_qa_ready_distribution::run(args.collect()),
        Some("manual-qa-ready-license") => manual_qa_ready_license::run(args.collect()),
        Some("manual-qa-ready-local-proof") => manual_qa_ready_local_proof::run(args.collect()),
        Some("media-policy-check") => media_policy_check::run(),
        Some("normalize-dmg") => normalize_dmg::run(args.collect()),
        Some("privacy-policy-check") => privacy_policy_check::run(),
        Some("productization-status") => productization_status::run(args.collect()),
        Some("publish-check") => publish_check::run(args.collect()),
        Some("release-check") => release_check::run(),
        Some("release-notes-check") => release_notes_check::run(args.collect()),
        Some("release-notes-prepare") => release_notes_prepare::run(args.collect()),
        Some("signed-dmg-check") => signed_dmg_check::run(args.collect()),
        Some("signed-dmg-copy") => signed_dmg_copy::run(args.collect()),
        Some("signed-dmg-prepare") => signed_dmg_prepare::run(args.collect()),
        Some("website-check") => website_check::run(args.collect()),
        _ => usage(),
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn usage() -> Result<(), String> {
    eprintln!(
        "usage: cargo run -p xtask -- <artifact-check|benchmark|benchmark-csv-check|checksum|file-size-check|github-release-plan|homebrew-cask|homebrew-cask-check|macos-codesign-plan|macos-codesign-verify-plan|macos-keychain-cleanup-plan|macos-keychain-plan|macos-notary-plan|macos-signing-check|macos-signing-plan|macos-spctl-plan|macos-stapler-plan|manual-qa-bad-input|manual-qa-check|manual-qa-clean-draft|manual-qa-fill-benchmark|manual-qa-fill-benchmark-threshold|manual-qa-fill-check|manual-qa-fill-local-proof|manual-qa-fill-release-gates|manual-qa-installed-app|manual-qa-link-samples|manual-qa-merge-prepared|manual-qa-pending|manual-qa-prepare|manual-qa-ready-all|manual-qa-ready-distribution|manual-qa-ready-license|manual-qa-ready-local-proof|media-policy-check|normalize-dmg|privacy-policy-check|productization-status|publish-check|release-check|release-notes-check|release-notes-prepare|signed-dmg-check|signed-dmg-copy|signed-dmg-prepare|website-check> [files...]"
    );
    std::process::exit(2);
}
