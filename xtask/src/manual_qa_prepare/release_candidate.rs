use std::path::Path;

const ARTIFACT_CHECK: &str = "`cargo run -p xtask -- artifact-check path/to/DropSquash.dmg`";
const CHECKSUM: &str = "`cargo run -p xtask -- checksum path/to/DropSquash.dmg`";

pub(super) fn print_rows(path: &Path) -> Result<(), String> {
    let rows = rows(path)?;
    if rows.is_empty() {
        return Ok(());
    }
    println!("manual QA Release Candidate rows:");
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

fn rows(path: &Path) -> Result<Vec<String>, String> {
    if path.extension().and_then(|value| value.to_str()) != Some("dmg") {
        return Ok(Vec::new());
    }
    crate::artifact_check::read_checked(path, "manual QA release candidate artifact")?;
    let checksum = crate::checksum::checksum_line(path)?;
    Ok(vec![
        format!(
            "| {ARTIFACT_CHECK} | Passes | artifact-check passed for {} DropSquash.dmg |",
            path.display()
        ),
        format!(
            "| {CHECKSUM} | SHA-256 line recorded | SHA-256 {checksum} for {} |",
            path.display()
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::{print_rows, rows};

    #[test]
    fn accepts_checked_dmg_artifact() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("DropSquash.dmg");
        std::fs::write(&path, dmg_bytes(b"dropsquash")).unwrap();

        let rows = rows(&path).unwrap();

        assert_eq!(rows.len(), 2);
        assert!(rows[0].contains("artifact-check passed"));
        assert!(rows[0].contains(path.to_str().unwrap()));
        assert!(rows[1].contains("SHA-256"));
        assert!(rows[1].contains("DropSquash.dmg"));
        print_rows(&path).unwrap();
    }

    #[test]
    fn rejects_dmg_with_nix_reference() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("DropSquash.dmg");
        std::fs::write(&path, dmg_bytes(b"/nix/store/abc")).unwrap();

        let error = print_rows(&path).unwrap_err();

        assert!(error.contains("/nix/store"));
    }

    #[test]
    fn generated_rows_satisfy_manual_qa_artifact_checks() {
        let directory = tempfile::tempdir().unwrap();
        let artifact = directory.path().join("DropSquash.dmg");
        std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
        let path = directory.path().join("manual-qa.md");
        std::fs::write(
            &path,
            format!(
                "| App artifact | {} |\n{}\n",
                artifact.display(),
                rows(&artifact).unwrap().join("\n")
            ),
        )
        .unwrap();

        let missing = crate::manual_qa_check::check_file(&path).unwrap();

        assert!(!missing.iter().any(|error| error.contains("artifact-check")));
        assert!(!missing.iter().any(|error| error.contains("checksum")));
    }

    fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
        let mut bytes = prefix.to_vec();
        let mut trailer = vec![0; 512];
        trailer[..4].copy_from_slice(b"koly");
        bytes.extend(trailer);
        bytes
    }
}
