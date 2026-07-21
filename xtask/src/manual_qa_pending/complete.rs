use super::packaged_app::benchmark_csv;

pub(crate) fn print(section: Option<&str>, text: &str) {
    match section {
        Some(filter) => {
            println!("manual QA has no pending result rows in {filter}");
            println!("{}", gate_for(filter));
            if let Some(note) = note_for(filter) {
                println!("{note}");
            }
            if let Some(reminder) = reminder_for(filter) {
                println!(
                    "{}",
                    crate::manual_qa_observation::packaged_visibility_reminder(reminder)
                );
            }
            if let Some(command) = command_for(filter, text) {
                println!("{command}");
            }
        }
        None => println!("manual QA has no pending result rows"),
    }
}

fn gate_for(filter: &str) -> String {
    format!(
        "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section {filter}"
    )
}

fn note_for(filter: &str) -> Option<&'static str> {
    match filter {
        "packaged-app" => Some(
            "packaged-app completion note: if the paid-beta blocker is still open, rerun these rows against the tested public DropSquash.dmg and use the sample-link and installed-app helper commands from manual-qa-ready-local-proof before chooser or mounted-DMG checks",
        ),
        _ => None,
    }
}

fn reminder_for(filter: &str) -> Option<&'static str> {
    match filter {
        "packaged-app" => Some("packaged-app observation reminder"),
        _ => None,
    }
}

fn command_for(filter: &str, text: &str) -> Option<String> {
    (filter == "packaged-app")
        .then(|| benchmark_csv(text))
        .flatten()
        .map(|csv| {
            format!(
                "packaged-app rerun helper: cargo run -p xtask -- manual-qa-ready-local-proof docs/manual-qa.md '{}'",
                csv.replace('\'', "'\\''")
            )
        })
}

#[cfg(test)]
mod tests {
    use super::{command_for, gate_for, note_for, reminder_for};

    #[test]
    fn prints_focused_gate_for_local_proof() {
        assert_eq!(
            gate_for("local-proof"),
            "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section local-proof"
        );
    }

    #[test]
    fn prints_packaged_app_completion_note() {
        assert!(note_for("packaged-app")
            .unwrap()
            .contains("tested public DropSquash.dmg"));
    }

    #[test]
    fn prints_packaged_app_observation_reminder() {
        let reminder = crate::manual_qa_observation::packaged_visibility_reminder(
            reminder_for("packaged-app").unwrap(),
        );

        assert!(reminder.contains("mounted app pid count"));
        assert!(reminder.contains("license field"));
        assert!(reminder.contains("Choose recording action"));
    }

    #[test]
    fn prints_packaged_app_rerun_helper_when_csv_exists() {
        let directory = tempfile::tempdir().unwrap();
        let csv = directory.path().join("results.csv");
        std::fs::write(&csv, "backend,input,output\n").unwrap();
        assert!(command_for(
            "packaged-app",
            &format!(
                "| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
                csv.display()
            )
        )
        .unwrap()
        .contains("manual-qa-ready-local-proof docs/manual-qa.md"));
    }
}
