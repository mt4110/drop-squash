use std::path::Path;

const ARTIFACT_CHECK: &str = "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`";
const CHECKSUM: &str =
    "`cargo run -p xtask -- checksum path/to/DropSquash.dmg --output SHA256SUMS`";

pub(super) fn print_rows(path: &Path, output_dir: &Path) -> Result<(), String> {
    let rows = rows(path)?;
    if rows.is_empty() {
        return Ok(());
    }
    println!("manual QA Checksum command:");
    println!("{}", checksum_command(path, output_dir));
    println!("manual QA Release Candidate rows:");
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub(super) fn rows(path: &Path) -> Result<Vec<String>, String> {
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return Ok(Vec::new());
    }
    crate::artifact_check::read_checked(path, "manual QA release candidate artifact")?;
    let checksum = crate::checksum::checksum_line(path)?;
    Ok(vec![
        format!(
            "| {ARTIFACT_CHECK} | Passes | artifact-check passed for public UDIF {} DropSquash.dmg |",
            path.display()
        ),
        format!(
            "| {CHECKSUM} | SHA-256 line recorded | SHA256SUMS created with lowercase SHA-256 {checksum} for {} |",
            path.display()
        ),
    ])
}

fn checksum_command(path: &Path, output_dir: &Path) -> String {
    let output = output_dir.join("SHA256SUMS");
    format!(
        "cargo run -p xtask -- checksum {} --output {}",
        path.display(),
        output.display()
    )
}

#[cfg(test)]
mod tests;
