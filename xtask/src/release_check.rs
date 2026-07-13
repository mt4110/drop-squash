use crate::{media_policy_check, privacy_policy_check, website_check};
mod blockers;
mod changelog;
mod desktop_capability;
mod dev_environment;
mod evidence;
mod manual_blockers;
mod productization;
mod release_doc;
mod release_notes;
mod required_text;
mod secret_files;
mod tauri_config;
mod workflow;

use std::path::Path;

pub fn run() -> Result<(), String> {
    dev_environment::reject_parallel_version_manager(Path::new("."))?;
    secret_files::reject_secret_files(Path::new("."))?;
    secret_files::require_local_agent_ignore(Path::new(".gitignore"))?;
    desktop_capability::check_default_capability(Path::new(
        "apps/desktop/src-tauri/capabilities/default.json",
    ))?;
    media_policy_check::check_default_roots()?;
    privacy_policy_check::check_default_roots()?;
    website_check::check_default_root()?;
    evidence::check_manual_only_coverage(
        Path::new("docs/qa-evidence.md"),
        Path::new("docs/manual-qa.md"),
    )?;
    blockers::check_release_blockers(Path::new("docs/release-blockers.md"))?;
    changelog::check(Path::new("CHANGELOG.md"))?;
    manual_blockers::check(
        Path::new("docs/release-blockers.md"),
        Path::new("docs/manual-qa.md"),
    )?;
    productization::check(Path::new("docs/productization.md"))?;
    release_doc::check(Path::new("docs/release.md"))?;
    release_notes::check(Path::new("docs/release-notes-template.md"))?;
    workflow::check_all()?;
    tauri_config::check(Path::new("apps/desktop/src-tauri/tauri.conf.json"))?;
    required_text::check()?;
    println!("release readiness checks passed");
    Ok(())
}

pub(crate) fn required_blockers() -> &'static [&'static str] {
    blockers::required()
}

pub(crate) fn blocker_completion_is_complete(blocker: &str, value: &str) -> bool {
    blockers::completion_is_complete(blocker, value)
}

#[cfg(test)]
mod tests;
