pub(super) fn validate(value: &str, file_name: &str, missing: &mut Vec<String>) {
    if mentions_drop_squash_state(value) && value.ends_with(file_name) {
        return;
    }
    missing.push(format!(
        "manual QA state path must end with DropSquash/{file_name}"
    ));
}

fn mentions_drop_squash_state(value: &str) -> bool {
    value.contains("Application Support/DropSquash/")
        || value.contains("Application Support\\DropSquash\\")
        || value.contains("$HOME/Library/Application Support/DropSquash/")
}
