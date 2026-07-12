mod artifact_check;
mod benchmark;
mod checksum;
mod dmg;
mod file_size_check;
mod homebrew_cask;
mod macos_signing_check;
mod manual_qa_check;
mod manual_qa_prepare;
mod media_policy_check;
mod normalize_dmg;
mod privacy_policy_check;
mod publish_check;
mod release_check;
mod release_notes_check;
mod release_notes_prepare;
mod release_url_fields;
mod website_check;

fn main() {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("artifact-check") => artifact_check::run(args.collect()),
        Some("benchmark") => benchmark::run(args.collect()),
        Some("checksum") => checksum::run(args.collect()),
        Some("file-size-check") => file_size_check::run(args.collect()),
        Some("homebrew-cask") => homebrew_cask::run(args.collect()),
        Some("manual-qa-check") => manual_qa_check::run(args.collect()),
        Some("manual-qa-prepare") => manual_qa_prepare::run(args.collect()),
        Some("macos-signing-check") => macos_signing_check::run(),
        Some("media-policy-check") => media_policy_check::run(),
        Some("normalize-dmg") => normalize_dmg::run(args.collect()),
        Some("privacy-policy-check") => privacy_policy_check::run(),
        Some("publish-check") => publish_check::run(args.collect()),
        Some("release-check") => release_check::run(),
        Some("release-notes-check") => release_notes_check::run(args.collect()),
        Some("release-notes-prepare") => release_notes_prepare::run(args.collect()),
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
        "usage: cargo run -p xtask -- <artifact-check|benchmark|checksum|file-size-check|homebrew-cask|manual-qa-check|manual-qa-prepare|macos-signing-check|media-policy-check|normalize-dmg|privacy-policy-check|publish-check|release-check|release-notes-check|release-notes-prepare|website-check> [files...]"
    );
    std::process::exit(2);
}
