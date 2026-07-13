use std::path::Path;

mod gates;

const CI_WORKFLOW: &str = ".github/workflows/ci.yml";
const DESKTOP_WORKFLOW: &str = ".github/workflows/desktop-ci.yml";
const RELEASE_WORKFLOW: &str = ".github/workflows/release.yml";
const SECURITY_WORKFLOW: &str = ".github/workflows/security.yml";

pub(super) fn check_all() -> Result<(), String> {
    check_workflow(CI_WORKFLOW, "CI", gates::CI, &[])?;
    check_workflow(DESKTOP_WORKFLOW, "desktop CI", gates::DESKTOP, &[])?;
    check_workflow(SECURITY_WORKFLOW, "security", gates::SECURITY, &[])?;
    check_workflow(
        RELEASE_WORKFLOW,
        "release",
        gates::RELEASE,
        gates::RELEASE_FORBIDDEN,
    )
}

fn check_workflow(
    path: &str,
    label: &str,
    gates: &[&'static str],
    forbidden: &[&'static str],
) -> Result<(), String> {
    let text = std::fs::read_to_string(Path::new(path)).map_err(|error| error.to_string())?;
    let missing = missing_workflow_gates(&text, gates);
    if !missing.is_empty() {
        return Err(format!(
            "{path} is missing {label} gates: {}",
            missing.join(", ")
        ));
    }
    let found = forbidden_workflow_values(&text, forbidden);
    if found.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{path} has forbidden {label} values: {}",
        found.join(", ")
    ))
}

fn missing_workflow_gates(text: &str, gates: &[&'static str]) -> Vec<&'static str> {
    gates
        .iter()
        .copied()
        .filter(|gate| !text.contains(gate))
        .collect()
}

fn forbidden_workflow_values(text: &str, forbidden: &[&'static str]) -> Vec<&'static str> {
    forbidden
        .iter()
        .copied()
        .filter(|value| text.contains(value))
        .collect()
}

#[cfg(test)]
pub(super) fn missing_ci_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, gates::CI)
}

#[cfg(test)]
pub(super) fn missing_release_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, gates::RELEASE)
}

#[cfg(test)]
pub(super) fn forbidden_release_workflow_values(text: &str) -> Vec<&'static str> {
    forbidden_workflow_values(text, gates::RELEASE_FORBIDDEN)
}

#[cfg(test)]
pub(super) fn missing_desktop_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, gates::DESKTOP)
}

#[cfg(test)]
pub(super) fn missing_security_workflow_gates(text: &str) -> Vec<&'static str> {
    missing_workflow_gates(text, gates::SECURITY)
}
