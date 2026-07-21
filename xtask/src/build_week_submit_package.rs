mod scan;

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

const DMG: &str = "target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg";
const FILES: [&str; 12] = [
    "README.md",
    "docs/build-week-submit-packet.md",
    "docs/build-week-devpost-draft.md",
    "docs/build-week-devpost-fields.md",
    "docs/build-week-devpost-checklist.md",
    "docs/build-week-final-entry-sheet.md",
    "docs/build-week-judge-runbook.md",
    "docs/build-week-phase5-evidence.json",
    "docs/phase5-alpha.md",
    "docs/phase5-native-bridge-qa.md",
    "docs/phase5-native-selective-pipeline.md",
    "docs/secure-share-support-matrix.md",
];

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err("usage: cargo run -p xtask -- build-week-submit-package".into());
    }
    crate::build_week_local_submit_check::run(Vec::new())?;
    let stamp = unix_stamp()?;
    let root = PathBuf::from(format!("/tmp/dropsquash-build-week-submit-{stamp}"));
    let zip = PathBuf::from(format!(
        "/tmp/dropsquash-build-week-submit-clean-{stamp}.zip"
    ));
    create_package(&root)?;
    scan::package(&root)?;
    zip_package(&root, &zip)?;
    println!("Build Week submit package created");
    println!("folder: {}", root.display());
    println!("zip: {}", zip.display());
    println!("zip_sha256: {}", sha256_file(&zip)?);
    Ok(())
}

fn create_package(root: &Path) -> Result<(), String> {
    std::fs::create_dir_all(root.join("docs")).map_err(|error| error.to_string())?;
    std::fs::copy(DMG, root.join("DropSquash_0.1.0_aarch64.dmg"))
        .map_err(|error| format!("{DMG}: {error}"))?;
    std::fs::write(
        root.join("DropSquash_0.1.0_aarch64.dmg.sha256"),
        format!(
            "{}  DropSquash_0.1.0_aarch64.dmg\n",
            sha256_file(Path::new(DMG))?
        ),
    )
    .map_err(|error| error.to_string())?;
    for file in FILES {
        let to = root.join(file);
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        std::fs::copy(file, &to).map_err(|error| format!("{file}: {error}"))?;
    }
    Ok(())
}

fn zip_package(root: &Path, zip: &Path) -> Result<(), String> {
    let parent = root
        .parent()
        .ok_or_else(|| format!("missing parent for {}", root.display()))?;
    let name = root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("invalid package name {}", root.display()))?;
    let output = Command::new("zip")
        .current_dir(parent)
        .args(["-qr", path(zip)?, name, "-x", "*/.DS_Store", "*/__MACOSX/*"])
        .output()
        .map_err(|error| format!("zip: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&output.stderr).to_string())
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn unix_stamp() -> Result<u64, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| error.to_string())
}

fn path(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path is not UTF-8: {}", path.display()))
}
