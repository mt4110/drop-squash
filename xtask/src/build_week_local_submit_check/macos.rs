use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn verify_distribution(dmg: &Path) -> Result<(), String> {
    run("codesign", &["--verify", "--verbose=4", path(dmg)?])?;
    run("xcrun", &["stapler", "validate", path(dmg)?])?;
    let mount = temp_mount()?;
    attach(dmg, &mount)?;
    let result = verify_mounted_app(&mount);
    let detach_result = detach(&mount);
    result.and(detach_result)
}

fn verify_mounted_app(mount: &Path) -> Result<(), String> {
    let app = mount.join("DropSquash.app");
    run(
        "spctl",
        &["--assess", "--type", "exec", "--verbose=4", path(&app)?],
    )?;
    run(
        "codesign",
        &["--verify", "--deep", "--strict", "--verbose=4", path(&app)?],
    )
}

fn attach(dmg: &Path, mount: &Path) -> Result<(), String> {
    std::fs::create_dir_all(mount).map_err(|error| format!("create mount dir: {error}"))?;
    run(
        "hdiutil",
        &[
            "attach",
            path(dmg)?,
            "-mountpoint",
            path(mount)?,
            "-nobrowse",
            "-quiet",
        ],
    )
}

fn detach(mount: &Path) -> Result<(), String> {
    run("hdiutil", &["detach", path(mount)?, "-quiet"])
}

fn temp_mount() -> Result<PathBuf, String> {
    let mut path = std::env::temp_dir();
    path.push(format!("dsq-submit-check-{}", std::process::id()));
    Ok(path)
}

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "{} failed: {}{}",
        program,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

fn path(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path is not UTF-8: {}", path.display()))
}
