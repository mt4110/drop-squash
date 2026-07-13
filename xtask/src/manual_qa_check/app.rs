mod command_groups;
mod evidence;
mod groups;

pub(super) fn validate_result(label: &str, result: &str, missing: &mut Vec<String>) {
    let label = label.trim();
    let Some(groups) = groups::for_label(label).or_else(|| command_groups::for_label(label)) else {
        return;
    };
    let lower = result.to_ascii_lowercase();
    if groups
        .iter()
        .all(|group| group.iter().any(|needle| lower.contains(needle)))
        && evidence::ok(label, result)
    {
        return;
    }
    missing.push(format!(
        "manual QA {label} needs concrete packaged-app evidence"
    ));
}
