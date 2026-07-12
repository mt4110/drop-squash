use std::path::Path;

mod required;

pub(super) fn check(root: &Path, errors: &mut Vec<String>) {
    for (page, needle) in required::TEXT {
        require_page_text(root, page, needle, errors);
    }
}

fn require_page_text(root: &Path, page: &str, needle: &str, errors: &mut Vec<String>) {
    let path = root.join(page);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return;
    };
    if !text.contains(needle) {
        errors.push(format!("{page} is missing required text: {needle}"));
    }
}
