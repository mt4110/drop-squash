use std::path::Path;

use super::{path_policy, sample_set, Options};

pub(super) fn validate(options: &Options) -> Result<(), String> {
    if options.reset_trial && options.restore_state {
        return Err("--reset-trial and --restore-state cannot be combined".to_string());
    }
    if options.restore_state && has_prepare_only_input(options) {
        return Err("--restore-state only accepts --app-state-dir and --state-dir".to_string());
    }
    if options.reset_trial && options.input_sample_set.is_none() {
        return Err(
            "--reset-trial requires --input-sample-set with short, medium, and large recordings"
                .to_string(),
        );
    }
    if options
        .input_sample_set
        .as_deref()
        .is_some_and(|value| !sample_set::is_valid(value))
    {
        return Err(
            "--input-sample-set must mention local short, medium, and large recordings".to_string(),
        );
    }
    path_policy::require_outside_repo("--app-state-dir", &options.app_state_dir)?;
    validate_markdown_output(options)?;
    path_policy::require_outside_repo("--output-dir", &options.output_dir)?;
    path_policy::require_outside_repo("--state-dir", &options.state_dir)
}

fn has_prepare_only_input(options: &Options) -> bool {
    options.app_artifact.is_some()
        || options.input_sample_set.is_some()
        || options.markdown_output.is_some()
}

fn validate_markdown_output(options: &Options) -> Result<(), String> {
    if let Some(path) = &options.markdown_output {
        path_policy::require_outside_repo("--markdown-output", path)?;
        require_markdown_file(path)?;
    }
    Ok(())
}

fn require_markdown_file(path: &Path) -> Result<(), String> {
    if path.extension().and_then(|value| value.to_str()) == Some("md") {
        return Ok(());
    }
    Err("--markdown-output must point to a .md file".to_string())
}
