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
    if extension == Some("app") {
        validate_app(path, missing);
    }
    if extension == Some("dmg") {
        validate_dmg(path, missing);
    }
}

fn validate_app(path: &Path, missing: &mut Vec<String>) {
    if !path.is_dir() {
        missing.push("manual QA .app artifact must be a directory".to_string());
        return;
    }
    if path.file_name().and_then(|value| value.to_str()) != Some("DropSquash.app") {
        missing.push("manual QA .app artifact must be named DropSquash.app".to_string());
    }
}

fn validate_dmg(path: &Path, missing: &mut Vec<String>) {
    if !path.is_file() {
        missing.push("manual QA .dmg artifact must be a file".to_string());
        return;
    }
    if path.file_name().and_then(|value| value.to_str()) != Some("DropSquash.dmg") {
        missing.push("manual QA .dmg artifact must be named DropSquash.dmg".to_string());
    }
    if let Err(error) = dmg::read(path, "manual QA artifact") {
        missing.push(error);
    }
}
