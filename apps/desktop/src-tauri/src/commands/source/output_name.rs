use std::path::Path;

pub(super) fn belongs_to_source(source_path: &Path, output_path: &Path) -> bool {
    let Some(source_stem) = source_path.file_stem().and_then(|value| value.to_str()) else {
        return false;
    };
    let Some(output_name) = output_path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };
    output_name == format!("{source_stem}.squashed.mp4")
        || numbered_belongs_to_source(source_stem, output_name)
}

fn numbered_belongs_to_source(source_stem: &str, output_name: &str) -> bool {
    let Some(number) = output_name
        .strip_prefix(&format!("{source_stem}.squashed-"))
        .and_then(|value| value.strip_suffix(".mp4"))
    else {
        return false;
    };
    !number.is_empty() && number.chars().all(|value| value.is_ascii_digit())
}
