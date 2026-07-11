mod artifact_check;
mod checksum;
mod file_size_check;
mod homebrew_cask;
mod macos_signing_check;
mod media_policy_check;
mod privacy_policy_check;
mod release_check;
mod website_check;

fn main() {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("artifact-check") => artifact_check::run(args.collect()),
        Some("checksum") => checksum::run(args.collect()),
        Some("file-size-check") => file_size_check::run(args.collect()),
        Some("homebrew-cask") => homebrew_cask::run(args.collect()),
        Some("macos-signing-check") => macos_signing_check::run(),
        Some("media-policy-check") => media_policy_check::run(),
        Some("privacy-policy-check") => privacy_policy_check::run(),
        Some("release-check") => release_check::run(),
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
        "usage: cargo run -p xtask -- <artifact-check|checksum|file-size-check|homebrew-cask|macos-signing-check|media-policy-check|privacy-policy-check|release-check|website-check> [files...]"
    );
    std::process::exit(2);
}
