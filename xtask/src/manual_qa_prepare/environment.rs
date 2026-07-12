use super::options::Options;

mod system;

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
            system::macos_version()?,
            system::machine_name()?,
            options.output_dir.display().to_string(),
            options.app_state_dir.display().to_string(),
            system::tester_name()?,
            system::today()?,
        )
    }

    pub(super) fn manual_qa_lines(&self) -> Vec<String> {
        self.manual_qa_fields()
            .into_iter()
            .map(|(label, value)| format!("manual QA {label}: {value}"))
            .collect()
    }

    pub(super) fn manual_qa_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("macOS version", format!("macOS {}", self.macos_version)),
            ("Machine", self.machine.clone()),
            ("Output folder", self.output_folder.clone()),
            ("Config path", format!("{}/config.json", self.app_state_dir)),
            (
                "History path",
                format!("{}/history.jsonl", self.app_state_dir),
            ),
            (
                "License cache path",
                format!("{}/license.json", self.app_state_dir),
            ),
            ("Tester", self.tester.clone()),
            ("Date", self.date.clone()),
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

#[cfg(test)]
mod tests;
