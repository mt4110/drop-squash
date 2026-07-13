mod artifact;
mod build_identity;
mod environment;
mod markdown;
mod options;
mod output;
mod prepared_markdown;
mod release_candidate;
mod state;

use artifact::qa_artifact;
use options::Options;
use state::{backup_state, restore_state, RESET_FILES};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    if options.restore_state {
        let restored = restore_state(&options)?;
        output::print_paths(&options)?;
        for file in restored {
            println!("restored: {file}");
        }
        return Ok(());
    }
    require_reset_artifact(&options)?;
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
    ]
}

#[cfg(test)]
mod tests;
