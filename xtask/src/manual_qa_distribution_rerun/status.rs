use std::path::{Path, PathBuf};

const ARTIFACT: &str = "target/release/bundle/dmg/DropSquash.dmg";
const BUNDLE_DIR: &str = "target/release/bundle/dmg";
const CHECKSUM: &str = "/tmp/dropsquash-unsigned-checksum/SHA256SUMS";
const FRESH_ARTIFACT: &str = "/tmp/dsq-build-target/release/bundle/dmg/DropSquash.dmg";
const FRESH_APP: &str = "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app";

pub(crate) fn lines() -> Vec<String> {
    lines_for_paths(Path::new(ARTIFACT), Path::new(CHECKSUM))
}

fn lines_for_paths(artifact: &Path, checksum: &Path) -> Vec<String> {
    let mut lines = vec![
        format!("unsigned dmg normalize command: cargo run -p xtask -- normalize-dmg {BUNDLE_DIR}"),
        format!("unsigned dmg artifact check: cargo run -p xtask -- artifact-check {ARTIFACT}"),
        format!(
            "unsigned dmg checksum command: cargo run -p xtask -- checksum {ARTIFACT} --output {CHECKSUM}"
        ),
    ];
    if !artifact.exists() {
        lines.push(format!(
            "unsigned dmg status: missing {}; run normalize command first",
            artifact.display()
        ));
        return lines;
    }
    lines.push(format!("unsigned dmg status: found {}", artifact.display()));
    lines.push(unsigned_artifact_status(artifact));
    lines.push(unsigned_checksum_status(artifact, checksum));
    lines.push(fresh_build_app_status());
    lines.push(fresh_build_dmg_status());
    lines
}

fn unsigned_artifact_status(artifact: &Path) -> String {
    match crate::artifact_check::read_checked(artifact, "unsigned dmg status") {
        Ok(_) => "unsigned dmg artifact status: artifact-check would pass".to_string(),
        Err(error) => format!("unsigned dmg artifact status: {error}"),
    }
}

fn unsigned_checksum_status(artifact: &Path, checksum: &Path) -> String {
    let existing = if checksum.exists() {
        format!("existing {}", checksum.display())
    } else {
        format!(
            "missing {}; run checksum command to create it",
            checksum.display()
        )
    };
    match crate::checksum::checksum_line(&PathBuf::from(artifact)) {
        Ok(line) => format!("unsigned dmg checksum status: {existing}; preview {line}"),
        Err(error) => {
            format!("unsigned dmg checksum status: {existing}; preview unavailable: {error}")
        }
    }
}

fn fresh_build_app_status() -> String {
    if Path::new(FRESH_APP).exists() {
        return format!("fresh build app status: found {FRESH_APP}");
    }
    format!("fresh build app status: missing {FRESH_APP}")
}

fn fresh_build_dmg_status() -> String {
    if Path::new(FRESH_ARTIFACT).exists() {
        return format!("fresh build dmg status: found {FRESH_ARTIFACT}");
    }
    format!("fresh build dmg status: missing {FRESH_ARTIFACT}")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::lines_for_paths;

    #[test]
    fn reports_missing_unsigned_artifact() {
        let lines = lines_for_paths(
            Path::new("/tmp/does-not-exist/DropSquash.dmg"),
            Path::new("/tmp/does-not-exist/SHA256SUMS"),
        );
        assert!(lines
            .iter()
            .any(|line| line.contains("missing /tmp/does-not-exist/DropSquash.dmg")));
    }

    #[test]
    fn reports_existing_unsigned_artifact_and_checksum_file() {
        let directory = tempfile::tempdir().unwrap();
        let artifact = directory.path().join("DropSquash.dmg");
        let checksum = directory.path().join("SHA256SUMS");
        fs::write(&artifact, b"not a dmg").unwrap();
        fs::write(&checksum, "checksum").unwrap();
        let lines = lines_for_paths(&artifact, &checksum);
        assert!(lines
            .iter()
            .any(|line| line.contains("unsigned dmg status: found")));
        assert!(lines
            .iter()
            .any(|line| line.contains("unsigned dmg artifact status:")));
        assert!(lines
            .iter()
            .any(|line| line.contains("checksum status: existing")));
        assert!(lines
            .iter()
            .any(|line| line.contains("fresh build app status:")));
        assert!(lines
            .iter()
            .any(|line| line.contains("fresh build dmg status:")));
    }
}
