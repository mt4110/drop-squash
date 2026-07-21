pub(crate) fn packaged_visibility_reminder(prefix: &str) -> String {
    format!(
        "{prefix}: confirm relaunch returns focus to the existing mounted-DMG window without increasing the mounted app pid count, and the disk-image notice, license field, and Choose recording action stay fully visible without clipping"
    )
}

pub(crate) fn packaged_visibility_note(prefix: &str) -> String {
    format!(
        "{prefix}: when the trial banner, license input, and install notice are visible together, confirm the full license field and the full Choose recording action remain visible without clipping"
    )
}
