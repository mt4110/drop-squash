mod artifact_check;
mod checksum;
mod homebrew_cask;
mod macos_signing_check;
mod release_check;

fn main() {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("artifact-check") => artifact_check::run(args.collect()),
        Some("checksum") => checksum::run(args.collect()),
        Some("homebrew-cask") => homebrew_cask::run(args.collect()),
        Some("macos-signing-check") => macos_signing_check::run(),
        Some("release-check") => release_check::run(),
        _ => usage(),
    };

    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn usage() -> Result<(), String> {
    eprintln!(
        "usage: cargo run -p xtask -- <artifact-check|checksum|homebrew-cask|macos-signing-check|release-check> [files...]"
    );
    std::process::exit(2);
}
