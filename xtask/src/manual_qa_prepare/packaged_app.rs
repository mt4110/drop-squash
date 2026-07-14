pub(super) fn print_rows() {
    println!("manual QA Packaged App observation rows:");
    for row in rows() {
        println!("{row}");
    }
}

pub(super) fn rows() -> Vec<String> {
    vec![
        "| Disk image launch notice | Launch from mounted `DropSquash.dmg` before copying to Applications | App warns that it is running from the disk image; Move copies `DropSquash.app` to `/Applications` without replacing an existing app, reveals the copied app in Finder, keeps a post-copy notice visible, opens the installed app on request, can request mounted-volume eject and quit the disk image copy, and does not delete the downloaded `.dmg` |  |".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::rows;

    #[test]
    fn generated_row_label_is_required_manual_qa_check() {
        let row = rows().join("\n");

        assert!(crate::manual_qa_check::requirements::REQUIRED_CHECKS
            .contains(&"Disk image launch notice"));
        assert!(row.contains("| Disk image launch notice |"));
    }

    #[test]
    fn generated_row_label_exists_in_manual_qa_template() {
        let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();

        assert!(template.contains("| Disk image launch notice |"));
    }
}
