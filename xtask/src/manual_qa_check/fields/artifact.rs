use crate::dmg;
use std::path::Path;

pub(super) fn validate(value: &str, missing: &mut Vec<String>) {
    let path = Path::new(value);
    if !path.exists() {
        missing.push(format!("manual QA artifact does not exist: {value}"));
    }
    let extension = path.extension().and_then(|value| value.to_str());
    if !matches!(extension, Some("app" | "dmg")) {
        missing.push("manual QA App artifact must be a .app or .dmg".to_string());
    }
    if extension == Some("app") && !path.is_dir() {
        missing.push("manual QA .app artifact must be a directory".to_string());
    }
    if extension == Some("dmg") {
        validate_dmg(path, missing);
    }
}

fn validate_dmg(path: &Path, missing: &mut Vec<String>) {
    if !path.is_file() {
        missing.push("manual QA .dmg artifact must be a file".to_string());
        return;
    }
    if let Err(error) = dmg::read(path, "manual QA artifact") {
        missing.push(error);
    }
}
