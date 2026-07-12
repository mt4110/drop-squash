mod artifact;
mod build_identity;
mod environment;
mod markdown;
mod options;
mod prepared_markdown;
mod release_candidate;
mod state;

use artifact::qa_artifact;
use build_identity::BuildIdentity;
use environment::Environment;
use markdown::Field;
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
    Err(
        "manual QA --reset-trial requires an existing DropSquash.app or DropSquash.dmg artifact"
            .to_string(),
    )
}

fn print_paths(options: &Options) -> Result<(), String> {
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
    let artifact = qa_artifact(options)?;
    let mut fields: Vec<Field> = Vec::new();
    match BuildIdentity::current_for_artifact(artifact.as_deref()) {
        Ok(identity) => {
            let app_build = identity.app_build();
            println!("manual QA App build: {app_build}");
            fields.push(("App build", app_build));
        }
        Err(error) => println!("manual QA App build unavailable: {error}"),
    }
    if let Some(path) = &artifact {
        println!("manual QA App artifact: {}", path.display());
        fields.push(("App artifact", path.display().to_string()));
    } else {
        println!("manual QA App artifact unavailable: pass --app-artifact <path>");
    }
    println!("{}", sample_set_line(options));
    if let Some(sample_set) = &options.input_sample_set {
        fields.push(("Input sample set", sample_set.clone()));
    }
    let environment = Environment::current(options)?;
    for line in environment.manual_qa_lines() {
        println!("{line}");
    }
    fields.extend(environment.manual_qa_fields());
    markdown::print_fields(&fields);
    if let Some(path) = &artifact {
        release_candidate::print_rows(path)?;
    }
    if let Some(path) = &options.markdown_output {
        prepared_markdown::write(path, &fields, artifact.as_deref())?;
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
