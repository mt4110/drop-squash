pub(super) fn print_rows() {
    println!("manual QA Packaged App observation rows:");
    for row in rows() {
        println!("{row}");
    }
}

pub(super) fn rows() -> Vec<String> {
    vec![
        "| Disk image launch notice | Launch from mounted `DropSquash.dmg` before copying to Applications | App warns that it is running from the disk image; Move copies `DropSquash.app` to `/Applications` without replacing an existing app, reveals the copied app in Finder, keeps a post-copy notice visible, opens the installed app on request, can request mounted-volume eject and quit the disk image copy, and does not delete the downloaded `.dmg` |  |".to_string(),
        "| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |".to_string(),
        "| Drag-and-drop conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |".to_string(),
        "| Privacy receipt sidecar | Successful conversion | Creates matching `.privacy.json` with file names, `uploaded_bytes = 0`, and `metadata_policy = preserve` |  |".to_string(),
        "| Reveal privacy receipt | Successful conversion with receipts enabled | Finder opens with generated `.privacy.json` selected |  |".to_string(),
        "| Duplicate output naming | Same recording twice | Second output uses `.squashed-2.mp4` style numbered suffix |  |".to_string(),
        "| Cancellation | Large recording | App returns to ready after temp cleanup; no success history; trial count unchanged |  |".to_string(),
        "| Multi-file queue | Three recordings | 3 recordings queue with 1 active sequential conversion; unrelated failures do not block finished jobs |  |".to_string(),
        "| Queued job cancellation | Three recordings | Cancelling a waiting row marks it cancelled, it never starts, trial count is unchanged, and history shows no new success |  |".to_string(),
        "| Batch summary | Three recordings with at least one mixed outcome | Queue summary shows trial or license lock blocked jobs plus numeric finished count, saved bytes, failed count, cancelled count, and blocked count |  |".to_string(),
        "| Ask source policy | Successful conversion | User can choose Trash or Keep while original remains unchanged |  |".to_string(),
        "| Trash source policy | Successful conversion | Trash button shows moving/disabled state; original moves to Trash only after verified smaller output |  |".to_string(),
        "| Failed conversion | Unsupported or intentionally bad input | Friendly error appears; original remains; trial count unchanged |  |".to_string(),
        "| Larger output | Input that cannot be made smaller | Larger/not-smaller result is treated as failure; original remains; trial count unchanged |  |".to_string(),
        "| Reveal output | Completed output link | Finder opens with generated `.squashed.mp4` selected |  |".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::rows;

    #[test]
    fn generated_row_labels_are_required_manual_qa_checks() {
        let rows = rows().join("\n");

        for check in [
            "Disk image launch notice",
            "Choose recording conversion",
            "Drag-and-drop conversion",
            "Privacy receipt sidecar",
            "Reveal privacy receipt",
            "Duplicate output naming",
            "Cancellation",
            "Multi-file queue",
            "Queued job cancellation",
            "Batch summary",
            "Ask source policy",
            "Trash source policy",
            "Failed conversion",
            "Larger output",
            "Reveal output",
        ] {
            assert!(crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&check));
            assert!(rows.contains(check));
        }
    }

    #[test]
    fn generated_row_labels_exist_in_manual_qa_template() {
        let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();

        for row in rows() {
            let check = row.split('|').nth(1).unwrap().trim();
            assert!(template.contains(check));
        }
    }
}
