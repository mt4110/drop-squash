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
    let copied = backup_state(&options)?;
    print_paths(&options)?;
    for file in copied {
        println!("copied: {file}");
    }
    if options.reset_trial {
        println!("trial state reset: {}", RESET_FILES.join(", "));
    }
    Ok(())
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
    if let Some(sample_set) = &options.input_sample_set {
        println!("manual QA Input sample set: {sample_set}");
    }
    if let Ok(environment) = Environment::current(options) {
        for line in environment.manual_qa_lines() {
            println!("{line}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
