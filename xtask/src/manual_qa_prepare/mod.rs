mod artifact;
mod build_identity;
mod environment;
mod options;
mod state;

use artifact::qa_artifact;
use build_identity::BuildIdentity;
use environment::Environment;
use options::Options;
use state::{backup_state, restore_state, RESET_FILES};

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    if options.restore_state {
        let restored = restore_state(&options)?;
        print_paths(&options)?;
        for file in restored {
            println!("restored: {file}");
        }
        return Ok(());
    }
    require_reset_artifact(&options)?;
    let copied = backup_state(&options)?;
    print_paths(&options)?;
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
    Err("manual QA --reset-trial requires an existing .app or .dmg artifact".to_string())
}

fn print_paths(options: &Options) -> Result<(), String> {
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
    match BuildIdentity::current() {
        Ok(identity) => println!("manual QA App build: {}", identity.app_build()),
        Err(error) => println!("manual QA App build unavailable: {error}"),
    }
    let artifact = qa_artifact(options)?;
    if let Some(path) = artifact {
        println!("manual QA App artifact: {}", path.display());
    } else {
        println!("manual QA App artifact unavailable: pass --app-artifact <path>");
    }
    println!("{}", sample_set_line(options));
    if let Ok(environment) = Environment::current(options) {
        for line in environment.manual_qa_lines() {
            println!("{line}");
        }
    }
    Ok(())
}

fn sample_set_line(options: &Options) -> String {
    match &options.input_sample_set {
        Some(sample_set) => format!("manual QA Input sample set: {sample_set}"),
        None => {
            "manual QA Input sample set unavailable: pass --input-sample-set <text>".to_string()
        }
    }
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
