mod artifact;
mod benchmark;
mod build_identity;
mod commands;
mod environment;
mod git_state;
mod license_sandbox;
mod markdown;
mod options;
mod output;
mod output_helper;
mod packaged_app;
mod prepared_markdown;
mod release_candidate;
mod release_gate;
mod state;

use artifact::qa_artifact;
use options::Options;
use state::{backup_state, restore_state, RESET_FILES};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    if options.restore_state {
        let restored = restore_state(&options)?;
        output::print_basic_paths(&options);
        for file in restored {
            println!("restored: {file}");
        }
        return Ok(());
    }
    git_state::require_clean_worktree()?;
    require_reset_artifact(&options)?;
    output::require_ready(&options)?;
    let copied = backup_state(&options)?;
    output::print_paths(&options)?;
    for file in copied {
        println!("copied: {file}");
    }
    if options.reset_trial {
        for line in reset_trial_lines(&options) {
            println!("{line}");
        }
    }
    Ok(())
}

fn require_reset_artifact(options: &Options) -> Result<(), String> {
    if !options.reset_trial {
        return Ok(());
    }
    if qa_artifact(options)?.is_some() {
        return Ok(());
    }
    Err(
        "manual QA --reset-trial requires an existing DropSquash.app or DropSquash.dmg artifact"
            .to_string(),
    )
}

fn reset_trial_lines(options: &Options) -> Vec<String> {
    vec![
        format!(
            "trial state reset path: {}",
            options.app_state_dir.display()
        ),
        format!("trial state reset: {}", RESET_FILES.join(", ")),
        format!(
            "trial state restore command: {}",
            commands::restore_command(options)
        ),
    ]
}

#[cfg(test)]
mod tests;
