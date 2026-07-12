pub(super) type Field = (&'static str, String);

pub(super) fn print_fields(fields: &[Field]) {
    println!("manual QA Markdown fields:");
    for row in rows(fields) {
        println!("{row}");
    }
}

pub(super) fn rows(fields: &[Field]) -> Vec<String> {
    fields
        .iter()
        .map(|(label, value)| format!("| {label} | {value} |"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{rows, Field};

    #[test]
    fn field_type_accepts_manual_qa_rows() {
        let fields: Vec<Field> = vec![("App build", "DropSquash 0.1.0 git abc1234".into())];

        assert_eq!(fields[0].0, "App build");
    }

    #[test]
    fn generated_rows_satisfy_manual_qa_field_checks() {
        let directory = tempfile::tempdir().unwrap();
        let artifact = directory.path().join("DropSquash.dmg");
        let output = directory.path().join("output");
        std::fs::write(&artifact, dmg_bytes(b"dropsquash")).unwrap();
        std::fs::create_dir(&output).unwrap();
        let fields = vec![
            (
                "App build",
                format!("DropSquash 0.1.0 git {}", current_head()),
            ),
            ("App artifact", artifact.display().to_string()),
            (
                "Input sample set",
                "short, medium, and large local recordings".into(),
            ),
            ("macOS version", "macOS 26.5.2".into()),
            ("Machine", "MacBookPro18,4 arm64".into()),
            ("Output folder", output.display().to_string()),
            (
                "Config path",
                "/Users/me/Library/Application Support/DropSquash/config.json".into(),
            ),
            (
                "History path",
                "/Users/me/Library/Application Support/DropSquash/history.jsonl".into(),
            ),
            (
                "License cache path",
                "/Users/me/Library/Application Support/DropSquash/license.json".into(),
            ),
            ("Tester", "masaki".into()),
            ("Date", "2026-07-12".into()),
        ];
        let path = directory.path().join("manual-qa.md");
        std::fs::write(&path, rows(&fields).join("\n")).unwrap();

        let missing = crate::manual_qa_check::check_file(&path).unwrap();

        assert!(!missing.iter().any(|error| error.contains("manual QA App")));
        assert!(!missing
            .iter()
            .any(|error| error.contains("manual QA field")));
        assert!(!missing
            .iter()
            .any(|error| error.contains("manual QA state path")));
    }

    #[test]
    fn generated_field_labels_are_required_manual_qa_fields() {
        let fields = vec![
            ("App build", String::new()),
            ("App artifact", String::new()),
            ("Input sample set", String::new()),
            ("macOS version", String::new()),
            ("Machine", String::new()),
            ("Output folder", String::new()),
            ("Config path", String::new()),
            ("History path", String::new()),
            ("License cache path", String::new()),
            ("Tester", String::new()),
            ("Date", String::new()),
        ];
        let untracked = fields
            .iter()
            .map(|(label, _)| *label)
            .filter(|label| !crate::manual_qa_check::requirements::REQUIRED_FIELDS.contains(label))
            .collect::<Vec<_>>();

        assert!(untracked.is_empty(), "{untracked:?}");
    }

    fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
        let mut bytes = prefix.to_vec();
        let mut trailer = vec![0; 512];
        trailer[..4].copy_from_slice(b"koly");
        bytes.extend(trailer);
        bytes
    }

    fn current_head() -> String {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "--short=7", "HEAD"])
            .output()
            .unwrap();
        assert!(output.status.success());
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }
}
