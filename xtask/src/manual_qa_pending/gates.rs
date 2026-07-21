pub(crate) fn for_groups(groups: &[(&'static str, Vec<(String, String)>)]) -> Vec<String> {
    groups
        .iter()
        .map(|(name, _)| match *name {
            "Packaged App" => {
                "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section packaged-app".to_string()
            }
            "License Sandbox" => {
                "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section license".to_string()
            }
            "Benchmark Evidence" => {
                "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section benchmark".to_string()
            }
            "Distribution And Signing" => {
                "focused gate: cargo run -p xtask -- manual-qa-check docs/manual-qa.md --section distribution".to_string()
            }
            _ => String::new(),
        })
        .filter(|line| !line.is_empty())
        .collect()
}
