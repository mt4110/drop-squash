pub(super) fn for_label(label: &str) -> Option<&'static str> {
    match label {
        "Disk image launch notice" => Some("note: requires launch from mounted DropSquash.dmg"),
        "Failed conversion" => Some(
            "note: requires unsupported or intentionally bad input outside the benchmark sample set",
        ),
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
