pub(super) fn validate(line: usize, input: &str) -> Result<(), String> {
    if is_prior_output(input) {
        return Err(format!(
            "benchmark CSV line {line} input must be an original local recording, not a prior .squashed output"
        ));
    }
    Ok(())
}

fn is_prior_output(input: &str) -> bool {
    input
        .rsplit('/')
        .next()
        .unwrap_or(input)
        .to_ascii_lowercase()
        .contains(".squashed")
}
