pub(super) fn validate(value: &str, file_name: &str, missing: &mut Vec<String>) {
    let value = value.trim().trim_matches('`');
    if is_absolute_state_path(value) && value.ends_with(file_name) {
        return;
    }
    missing.push(format!(
        "manual QA state path must be an absolute DropSquash/{file_name} path"
    ));
}

fn is_absolute_state_path(value: &str) -> bool {
    (value.starts_with('/') || has_windows_drive_prefix(value))
        && (value.contains("Application Support/DropSquash/")
            || value.contains("Application Support\\DropSquash\\"))
}

fn has_windows_drive_prefix(value: &str) -> bool {
    value.as_bytes().get(1) == Some(&b':')
}
