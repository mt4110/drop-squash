use std::path::PathBuf;

mod path_policy;
mod sample_set;
mod validation;

#[derive(Debug)]
pub(super) struct Options {
    pub(super) app_artifact: Option<PathBuf>,
    pub(super) app_state_dir: PathBuf,
    pub(super) input_sample_set: Option<String>,
    pub(super) markdown_output: Option<PathBuf>,
    pub(super) output_dir: PathBuf,
    pub(super) reset_trial: bool,
    pub(super) restore_state: bool,
    pub(super) state_dir: PathBuf,
}

impl Options {
    pub(super) fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut options = Self::default()?;
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if matches!(arg.as_str(), "--help" | "-h") {
                return Err(usage());
            }
            if arg == "--reset-trial" {
                options.reset_trial = true;
                continue;
            }
            if arg == "--restore-state" {
                options.restore_state = true;
                continue;
            }
            let value = args
                .next()
                .ok_or_else(|| format!("{arg} requires a value\n{}", usage()))?;
            options.set_path(&arg, value)?;
        }
        validation::validate(&options)?;
        Ok(options)
    }

    fn set_path(&mut self, arg: &str, value: String) -> Result<(), String> {
        match arg {
            "--app-artifact" => self.app_artifact = Some(PathBuf::from(value)),
            "--app-state-dir" => self.app_state_dir = PathBuf::from(value),
            "--input-sample-set" => self.input_sample_set = Some(value),
            "--markdown-output" => self.markdown_output = Some(PathBuf::from(value)),
            "--output-dir" => self.output_dir = PathBuf::from(value),
            "--state-dir" => self.state_dir = PathBuf::from(value),
            other => return Err(format!("unknown manual QA prepare argument: {other}")),
        }
        Ok(())
    }

    fn default() -> Result<Self, String> {
        let home = std::env::var("HOME").map_err(|_| "HOME is not set".to_string())?;
        Ok(Self {
            app_artifact: None,
            app_state_dir: PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("DropSquash"),
            input_sample_set: None,
            markdown_output: None,
            output_dir: PathBuf::from("/tmp/dropsquash-manual-qa-output"),
            reset_trial: false,
            restore_state: false,
            state_dir: PathBuf::from("/tmp/dropsquash-qa-state"),
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- manual-qa-prepare [--reset-trial|--restore-state] [--app-artifact <path>] [--input-sample-set <text>] [--markdown-output <path>] [--state-dir <dir>] [--output-dir <dir>] [--app-state-dir <dir>]"
        .to_string()
}
