pub(super) fn for_manual(text: &str) -> Vec<String> {
    samples(text).map(|set| set.summary_lines()).unwrap_or_default()
}

pub(super) fn guidance_for(label: &str, text: &str) -> Option<String> {
    samples(text).and_then(|set| set.guidance(label))
}

fn samples(text: &str) -> Option<SampleSet> {
    table_result(text, "Benchmark sample set")
        .and_then(crate::csv_evidence::existing_outside_repo_path)
        .and_then(|csv| SampleSet::from_csv(&csv).ok())
}

struct SampleSet {
    small: String,
    medium: String,
    large: String,
}

impl SampleSet {
    fn from_csv(csv: &std::path::Path) -> Result<Self, String> {
        let rows = crate::benchmark_csv_check::read_rows(csv)?;
        let samples = rows.iter().skip(1).filter_map(|row| row.get(1)).collect::<Vec<_>>();
        let [small, medium, large] = samples.as_slice() else {
            return Err("benchmark CSV must contain exactly three sample rows".to_string());
        };
        Ok(Self {
            small: (*small).clone(),
            medium: (*medium).clone(),
            large: (*large).clone(),
        })
    }

    fn summary_lines(&self) -> Vec<String> {
        vec![
            format!("packaged-app small sample: {}", self.small),
            format!("packaged-app duplicate sample: {}", self.small),
            format!(
                "packaged-app queue sample set: {}, {}, {}",
                self.small, self.medium, self.large
            ),
            format!("packaged-app large sample: {}", self.large),
        ]
    }

    fn guidance(&self, label: &str) -> Option<String> {
        match label {
            "Choose recording conversion" | "Drag-and-drop conversion"
            | "Privacy receipt sidecar" | "Reveal privacy receipt"
            | "Ask source policy" | "Trash source policy" | "Reveal output" => {
                Some(format!("sample: small ({})", self.small))
            }
            "Duplicate output naming" => Some(format!("sample: duplicate ({})", self.small)),
            "Cancellation" | "Larger output" => {
                Some(format!("sample: large ({})", self.large))
            }
            "Multi-file queue" | "Queued job cancellation" | "Batch summary" => Some(format!(
                "sample: queue set ({}, {}, {})",
                self.small, self.medium, self.large
            )),
            _ => None,
        }
    }
}

fn table_result<'a>(text: &'a str, label: &str) -> Option<&'a str> {
    text.lines().find_map(|line| {
        let cells = line.trim_matches('|').split('|').map(str::trim).collect::<Vec<_>>();
        match cells.as_slice() {
            [row, _, result] | [row, _, _, result] if *row == label => Some(*result),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{for_manual, guidance_for};

    fn manual_text(csv: &std::path::Path) -> String {
        format!(
            "| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
            csv.display()
        )
    }

    fn write_csv() -> (tempfile::TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let csv = directory.path().join("results.csv");
        std::fs::write(
            &csv,
            "backend,input\napple-native,/tmp/short.mov\napple-native,/tmp/medium.mov\napple-native,/tmp/large.mov\n",
        )
        .unwrap();
        (directory, csv)
    }

    #[test]
    fn reports_hints_from_benchmark_sample_set_csv() {
        let (_directory, csv) = write_csv();
        let hints = for_manual(&manual_text(&csv));
        assert_eq!(hints[0], "packaged-app small sample: /tmp/short.mov");
        assert_eq!(hints[1], "packaged-app duplicate sample: /tmp/short.mov");
        assert_eq!(
            hints[2],
            "packaged-app queue sample set: /tmp/short.mov, /tmp/medium.mov, /tmp/large.mov"
        );
        assert_eq!(hints[3], "packaged-app large sample: /tmp/large.mov");
    }

    #[test]
    fn reports_row_specific_guidance() {
        let (_directory, csv) = write_csv();
        let text = manual_text(&csv);
        assert_eq!(
            guidance_for("Batch summary", &text).as_deref(),
            Some("sample: queue set (/tmp/short.mov, /tmp/medium.mov, /tmp/large.mov)")
        );
        assert_eq!(
            guidance_for("Trash source policy", &text).as_deref(),
            Some("sample: small (/tmp/short.mov)")
        );
    }
}
