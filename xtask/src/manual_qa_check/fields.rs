use std::path::Path;

mod artifact;
mod date;
mod environment;
mod state_path;

pub(super) fn validate(label: &str, value: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    match label {
        "App artifact" => artifact::validate(value, missing),
        "App build" => environment::validate_app_build(value, missing),
        "macOS version" => environment::validate_macos_version(value, missing),
        "Machine" => environment::validate_machine(value, missing),
        "Input sample set" => environment::validate_input_sample_set(value, missing),
        "Output folder" => validate_output_folder(value, missing),
        "Config path" => state_path::validate(value, "config.json", missing),
        "History path" => state_path::validate(value, "history.jsonl", missing),
        "License cache path" => state_path::validate(value, "license.json", missing),
        "Tester" => validate_tester(value, missing),
        "Date" if !date::is_iso(value) => {
            missing.push("manual QA Date must use YYYY-MM-DD".to_string());
        }
        _ => {}
    }
}

fn validate_output_folder(value: &str, missing: &mut Vec<String>) {
    let path = Path::new(value);
    if path.is_dir() {
        return;
    }
    missing.push(format!("manual QA Output folder must exist: {value}"));
}

fn validate_tester(value: &str, missing: &mut Vec<String>) {
    let lower = value.to_ascii_lowercase();
    if value.len() >= 3 && !lower.contains("concrete evidence") && lower != "tester" {
        return;
    }
    missing.push("manual QA Tester must name the tester".to_string());
}
