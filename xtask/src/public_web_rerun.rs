use std::process::Command;

#[cfg(test)]
mod tests;

const USAGE: &str = "usage: cargo run -p xtask -- public-web-rerun";
const OWNER_ONLY_URL: &str = "https://dropsquash-app.system-obj-gg.chatgpt.site";
const APEX_TARGETS: &str = "162.159.143.30 172.66.3.26";
const OPENAI_VERIFY: &str = "openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g";
const CF_VERIFY: &str = "c70e75c8-8887-4b4a-a390-72fd4a8400fd";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Err(USAGE.to_string());
    }
    let status = git_status().ok();
    for line in crate::manual_qa_dirty_worktree::quickstart_lines(status.as_deref()) {
        println!("{line}");
    }
    for line in state_lines(current_head().ok().as_deref(), status.as_deref()) {
        println!("{line}");
    }
    for line in next_lines() {
        println!("{line}");
    }
    Ok(())
}

fn state_lines(head: Option<&str>, status: Option<&str>) -> Vec<String> {
    let mut lines = Vec::new();
    if let Some(head) = head {
        lines.push(format!("public web source head: {head}"));
    }
    if let Some(status) = status.filter(|value| !crate::git_status::is_clean(value)) {
        lines.push(format!(
            "public web dirty paths: {}",
            crate::git_status::dirty_paths(status)
        ));
    }
    lines
}

pub(crate) fn next_lines() -> [String; 20] {
    [
        "public web ready: cargo run -p xtask -- public-web-ready".to_string(),
        "public web snapshot handoff: scripts/public-web-handoff.sh /tmp/dropsquash-public-web-$(git rev-parse --short HEAD)".to_string(),
        "public web gate: cargo run -p xtask -- website-check".to_string(),
        "deployable site gate: npm --prefix apps/site run verify:site".to_string(),
        "public web probe: cargo run -p xtask -- public-web-probe".to_string(),
        "public web track: cargo run -p xtask -- productization-status --track 'Public web proof'".to_string(),
        "public beta operator checklist: docs/public-beta-operator-checklist.md".to_string(),
        "website deployment runbook: docs/website-deployment-runbook.md (see `Short Execution Memo` for the fast path)".to_string(),
        "public web next step 1: deploy the saved Sites version that contains the updated public copy".to_string(),
        "public web next step 2: verify the final production URLs https://dropsquash.app/pricing and https://dropsquash.app/refund".to_string(),
        "public web next step 3: keep checkout blocked until the tested live https://store.lemonsqueezy.com/checkout/buy/<id> URL exists".to_string(),
        "dns probe: dig +short A dropsquash.app; dig +short TXT _openai-site-verification.dropsquash.app; dig +short TXT _cf-custom-hostname.dropsquash.app".to_string(),
        format!("canonical DNS targets: A dropsquash.app -> {APEX_TARGETS}"),
        format!(
            "canonical TXT targets: _openai-site-verification.dropsquash.app -> {OPENAI_VERIFY}; _cf-custom-hostname.dropsquash.app -> {CF_VERIFY}"
        ),
        format!(
            "owner-history note: {OWNER_ONLY_URL} is useful only for provenance; public blocker evidence must come from dropsquash.app"
        ),
        "canonical domain status note: dropsquash.app is live; the remaining blocker is stale pricing/refund copy until the saved version is deployed".to_string(),
        "canonical host note: if pricing or refund still shows draft-era copy, keep the public-web blockers open".to_string(),
        "public web production URLs: https://dropsquash.app/release-status https://dropsquash.app/pricing https://dropsquash.app/refund https://store.lemonsqueezy.com/checkout/buy/<id>".to_string(),
        "source-state note: save/deploy only from an intentional committed site source; a dirty worktree is preflight only".to_string(),
        "final public-release gate later: cargo run -p xtask -- publish-check /absolute/path/to/release-notes.md after production URLs, release notes, and blocker evidence exist".to_string(),
    ]
}

fn git_status() -> Result<String, String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git status failed".to_string());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn current_head() -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git rev-parse HEAD failed".to_string());
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_string())
        .map_err(|error| error.to_string())
}
