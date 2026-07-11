use std::process::Command;

use super::options::Options;

#[derive(Debug)]
pub(super) struct Environment {
    macos_version: String,
    machine: String,
    output_folder: String,
    app_state_dir: String,
    tester: String,
    date: String,
}

impl Environment {
    pub(super) fn current(options: &Options) -> Result<Self, String> {
        Self::from_parts(
            command_text("sw_vers", &["-productVersion"])?,
            machine_name()?,
            options.output_dir.display().to_string(),
            options.app_state_dir.display().to_string(),
            tester_name()?,
            command_text("date", &["+%F"])?,
        )
    }

    pub(super) fn manual_qa_lines(&self) -> Vec<String> {
        vec![
            format!("manual QA macOS version: macOS {}", self.macos_version),
            format!("manual QA Machine: {}", self.machine),
            format!("manual QA Output folder: {}", self.output_folder),
            format!("manual QA Config path: {}/config.json", self.app_state_dir),
            format!(
                "manual QA History path: {}/history.jsonl",
                self.app_state_dir
            ),
            format!(
                "manual QA License cache path: {}/license.json",
                self.app_state_dir
            ),
            format!("manual QA Tester: {}", self.tester),
            format!("manual QA Date: {}", self.date),
        ]
    }

    fn from_parts(
        macos_version: String,
        machine: String,
        output_folder: String,
        app_state_dir: String,
        tester: String,
        date: String,
    ) -> Result<Self, String> {
        if [
            macos_version.as_str(),
            machine.as_str(),
            output_folder.as_str(),
            app_state_dir.as_str(),
            tester.as_str(),
            date.as_str(),
        ]
        .iter()
        .any(|value| value.trim().is_empty())
        {
            return Err("manual QA environment fields must be non-empty".to_string());
        }
        Ok(Self {
            macos_version,
            machine,
            output_folder,
            app_state_dir,
            tester,
            date,
        })
    }
}

fn machine_name() -> Result<String, String> {
    let arch = command_text("uname", &["-m"])?;
    let model = command_text("sysctl", &["-n", "hw.model"]).unwrap_or_default();
    if model.is_empty() {
        return Ok(arch);
    }
    Ok(format!("{model} {arch}"))
}

fn tester_name() -> Result<String, String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .map_err(|_| "USER is not set".to_string())
}

fn command_text(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("{command} failed"));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}

#[cfg(test)]
mod tests;
