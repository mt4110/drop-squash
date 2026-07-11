use std::path::PathBuf;

#[derive(Debug)]
pub(super) struct Options {
    pub(super) app_state_dir: PathBuf,
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
        options.validate()?;
        Ok(options)
    }

    fn set_path(&mut self, arg: &str, value: String) -> Result<(), String> {
        match arg {
            "--app-state-dir" => self.app_state_dir = PathBuf::from(value),
            "--output-dir" => self.output_dir = PathBuf::from(value),
            "--state-dir" => self.state_dir = PathBuf::from(value),
            other => return Err(format!("unknown manual QA prepare argument: {other}")),
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        if self.reset_trial && self.restore_state {
            return Err("--reset-trial and --restore-state cannot be combined".to_string());
        }
        Ok(())
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
            restore_state: false,
            state_dir: PathBuf::from("/tmp/dropsquash-qa-state"),
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- manual-qa-prepare [--reset-trial|--restore-state] [--state-dir <dir>] [--output-dir <dir>] [--app-state-dir <dir>]"
        .to_string()
}
