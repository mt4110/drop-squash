mod guidance;

use super::fields::table_result;
const PANEL_DIR: &str = "/tmp/dropsquash-qa-open-panel";

#[cfg(test)]
mod tests;

pub(super) fn for_manual(text: &str) -> Vec<String> {
    samples(text)
        .map(|set| set.summary_lines())
        .unwrap_or_default()
}

pub(super) fn for_benchmark(text: &str) -> Vec<String> {
    samples(text)
        .map(|set| set.benchmark_lines())
        .unwrap_or_default()
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
    small_alias: String,
    medium_alias: String,
    large_alias: String,
    not_smaller: String,
}

impl SampleSet {
    fn from_csv(csv: &std::path::Path) -> Result<Self, String> {
        let rows = crate::benchmark_csv_check::read_rows(csv)?;
        let samples = rows.iter().skip(1).take(3).collect::<Vec<_>>();
        let [small, medium, large] = samples.as_slice() else {
            return Err("benchmark CSV must contain exactly three sample rows".to_string());
        };
        Ok(Self {
            small: row_input(small, "small")?,
            medium: row_input(medium, "medium")?,
            large: row_input(large, "large")?,
            small_alias: alias_name("small", small)?,
            medium_alias: alias_name("medium", medium)?,
            large_alias: alias_name("large", large)?,
            not_smaller: row_output(small, "small")?,
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
            format!("packaged-app not-smaller sample: {}", self.not_smaller),
            format!(
                "packaged-app sample aliases: {0}/{1}, {0}/{2}, {0}/{3}, {0}/qa-not-smaller.mp4",
                PANEL_DIR, self.small_alias, self.medium_alias, self.large_alias
            ),
        ]
    }

    fn benchmark_lines(&self) -> Vec<String> {
        vec![
            format!("benchmark release-set inputs: short={}, medium={}, large={}", self.small, self.medium, self.large),
            "benchmark release-set rule: use original local recordings for short/medium/large, not prior .squashed outputs".to_string(),
            "benchmark release-set rule: keep profile and size aligned with the shipping setting under test".to_string(),
            format!("benchmark release-set fallback: treat {}/qa-not-smaller.mp4 as a candidate only; if it still saves bytes under the current shipping profile and size, relink a different kept-original candidate before recording Larger output or rerunning the benchmark set", PANEL_DIR),
        ]
    }

    fn guidance(&self, label: &str) -> Option<String> {
        guidance::for_label(self, label)
    }
}

fn row_input(row: &[String], label: &str) -> Result<String, String> {
    row.get(1)
        .cloned()
        .ok_or_else(|| format!("benchmark CSV row is missing input column: {label}"))
}

fn row_output(row: &[String], label: &str) -> Result<String, String> {
    row.get(2)
        .cloned()
        .ok_or_else(|| format!("benchmark CSV row is missing output column: {label}"))
}

fn alias_name(label: &str, row: &[String]) -> Result<String, String> {
    let input = row_input(row, label)?;
    let extension = std::path::Path::new(&input)
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("benchmark CSV row is missing file extension: {label}"))?;
    Ok(format!("qa-{label}.{extension}"))
}
