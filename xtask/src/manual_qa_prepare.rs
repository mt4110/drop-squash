use std::path::PathBuf;

const RESET_FILES: [&str; 2] = ["history.jsonl", "license.json"];
const STATE_FILES: [&str; 3] = ["config.json", RESET_FILES[0], RESET_FILES[1]];

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    let copied = prepare(&options)?;
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
    for file in copied {
        println!("copied: {file}");
    }
    if options.reset_trial {
        println!("trial state reset: {}", RESET_FILES.join(", "));
    }
    Ok(())
}

fn prepare(options: &Options) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(&options.state_dir).map_err(|error| error.to_string())?;
    std::fs::create_dir_all(&options.output_dir).map_err(|error| error.to_string())?;
    let mut copied = Vec::new();
    for file in STATE_FILES {
        let source = options.app_state_dir.join(file);
        if !source.exists() {
            continue;
        }
        std::fs::copy(&source, options.state_dir.join(file)).map_err(|error| error.to_string())?;
        copied.push(file.to_string());
    }
    if options.reset_trial {
        reset_trial_state(options)?;
    }
    Ok(copied)
}

fn reset_trial_state(options: &Options) -> Result<(), String> {
    for file in RESET_FILES {
        let path = options.app_state_dir.join(file);
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Options {
    app_state_dir: PathBuf,
    output_dir: PathBuf,
    reset_trial: bool,
    state_dir: PathBuf,
}

impl Options {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut options = Self::default()?;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if arg == "--reset-trial" {
                options.reset_trial = true;
                continue;
            }
            let value = args
                .next()
                .ok_or_else(|| format!("{arg} requires a value\n{}", usage()))?;
            match arg.as_str() {
                "--app-state-dir" => options.app_state_dir = PathBuf::from(value),
                "--output-dir" => options.output_dir = PathBuf::from(value),
                "--state-dir" => options.state_dir = PathBuf::from(value),
                other => return Err(format!("unknown manual QA prepare argument: {other}")),
            }
        }
        Ok(options)
    }

    fn default() -> Result<Self, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
        Ok(Self {
            app_state_dir: PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("DropSquash"),
            output_dir: PathBuf::from("/tmp/dropsquash-manual-qa-output"),
            reset_trial: false,
            state_dir: PathBuf::from("/tmp/dropsquash-qa-state"),
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- manual-qa-prepare [--reset-trial] [--state-dir <dir>] [--output-dir <dir>] [--app-state-dir <dir>]"
        .to_string()
}

#[cfg(test)]
mod tests;
