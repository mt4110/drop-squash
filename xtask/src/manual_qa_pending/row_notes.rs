pub(super) fn for_label(label: &str) -> Option<&'static str> {
    match label {
        "Disk image launch notice" => Some("note: requires launch from mounted DropSquash.dmg"),
        "Failed conversion" => Some(
            "note: requires unsupported or intentionally bad input outside the benchmark sample set",
        ),
        "`cargo run -p dropsquash -- license status`" => {
            Some("note: record diagnostic lines without pasting the sandbox key")
        }
        "`cargo run -p xtask -- macos-signing-check`" => {
            Some("note: requires release signing environment variables and secrets")
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::for_label;

    #[test]
    fn reports_note_for_disk_image_launch() {
        assert_eq!(
            for_label("Disk image launch notice"),
            Some("note: requires launch from mounted DropSquash.dmg")
        );
    }
}
