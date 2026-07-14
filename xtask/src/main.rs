mod artifact_age;
mod artifact_check;
mod benchmark;
mod benchmark_csv_check;
mod checksum;
mod csv_evidence;
mod dmg;
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
mod manual_qa_check;
mod manual_qa_prepare;
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
        Some("manual-qa-check") => manual_qa_check::run(args.collect()),
        Some("manual-qa-prepare") => manual_qa_prepare::run(args.collect()),
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
        "usage: cargo run -p xtask -- <artifact-check|benchmark|benchmark-csv-check|checksum|file-size-check|github-release-plan|homebrew-cask|homebrew-cask-check|macos-codesign-plan|macos-codesign-verify-plan|macos-keychain-cleanup-plan|macos-keychain-plan|macos-notary-plan|macos-signing-check|macos-signing-plan|macos-spctl-plan|macos-stapler-plan|manual-qa-check|manual-qa-prepare|media-policy-check|normalize-dmg|privacy-policy-check|productization-status|publish-check|release-check|release-notes-check|release-notes-prepare|signed-dmg-check|signed-dmg-copy|signed-dmg-prepare|website-check> [files...]"
    );
    std::process::exit(2);
}
