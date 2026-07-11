use std::path::PathBuf;

const STATE_FILES: [&str; 3] = ["config.json", "history.jsonl", "license.json"];

pub fn run(args: Vec<String>) -> Result<(), String> {
    let options = Options::parse(args)?;
    let copied = prepare(&options)?;
    println!("manual QA state backup: {}", options.state_dir.display());
    println!("manual QA output folder: {}", options.output_dir.display());
    println!("app state source: {}", options.app_state_dir.display());
    for file in copied {
        println!("copied: {file}");
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
    Ok(copied)
}

#[derive(Debug)]
struct Options {
    app_state_dir: PathBuf,
    output_dir: PathBuf,
    state_dir: PathBuf,
}

impl Options {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut options = Self::default()?;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
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
            state_dir: PathBuf::from("/tmp/dropsquash-qa-state"),
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- manual-qa-prepare [--state-dir <dir>] [--output-dir <dir>] [--app-state-dir <dir>]"
        .to_string()
}

#[cfg(test)]
mod tests;
