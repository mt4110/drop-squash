pub(super) fn render(blocker: &str, next_action: &str, owner: &str) -> String {
    match blocker {
        "Packaged macOS manual QA" => packaged(blocker, next_action, owner),
        "Lemon Squeezy product setup"
        | "Lemon Squeezy sandbox purchase"
        | "Valid sandbox activation" => license(blocker, next_action, owner),
        "Signed DMG" | "Notarized and stapled DMG" | "Gatekeeper clean-machine open" => {
            distribution(blocker, next_action, owner)
        }
        "Public website deployment"
        | "Pricing finalized"
        | "Refund policy finalized"
        | "Live checkout link" => web(blocker, next_action, owner),
        "Published checksum" => checksum(blocker, next_action, owner),
        "Homebrew cask install" => cask(blocker, next_action, owner),
        _ => plain(blocker, next_action, owner),
    }
}

fn plain(blocker: &str, action: &str, owner: &str) -> String {
    format!("- {blocker}: {action} ({owner})")
}
fn packaged(blocker: &str, action: &str, owner: &str) -> String {
    format!(
        "{}; start with cargo run -p xtask -- manual-qa-packaged-rerun; {}",
        plain(blocker, action, owner),
        crate::manual_qa_observation::packaged_visibility_reminder("then")
    )
}
fn license(blocker: &str, action: &str, owner: &str) -> String {
    if action.contains("manual-qa-license-rerun") || action.contains("Short Execution Memo") {
        return plain(blocker, action, owner);
    }
    format!("{}; start with cargo run -p xtask -- manual-qa-license-rerun; see docs/license-sandbox-runbook.md `Short Execution Memo` for the fast path", plain(blocker, action, owner))
}
fn distribution(blocker: &str, action: &str, owner: &str) -> String {
    if action.contains("manual-qa-distribution-rerun") || action.contains("Short Execution Memo") {
        return plain(blocker, action, owner);
    }
    format!("{}; start with cargo run -p xtask -- manual-qa-distribution-rerun; see docs/signed-dmg-runbook.md `Short Execution Memo` for the fast path", plain(blocker, action, owner))
}
fn web(blocker: &str, action: &str, owner: &str) -> String {
    if action.contains("Short Execution Memo")
        || action.contains("docs/public-beta-operator-checklist.md")
    {
        return plain(blocker, action, owner);
    }
    let hint = if action.contains("public-web-probe") {
        "; see docs/public-beta-operator-checklist.md `Short Execution Memo` for the fast path"
    } else {
        "; start with cargo run -p xtask -- public-web-ready, use cargo run -p xtask -- public-web-rerun as the operator memo when needed, then keep publish-check for the final public-release gate after production URLs, release notes, and blocker evidence exist; see docs/public-beta-operator-checklist.md `Short Execution Memo` for the fast path"
    };
    format!("{}{hint}", plain(blocker, action, owner))
}
fn checksum(blocker: &str, action: &str, owner: &str) -> String {
    if action.contains("publish-check") || action.contains("Short Execution Memo") {
        return plain(blocker, action, owner);
    }
    format!("{}; start with cargo run -p xtask -- publish-check /absolute/path/to/release-notes.md after the public GitHub Release has the matching SHA256SUMS attachment; see docs/public-beta-operator-checklist.md `Short Execution Memo` for the fast path", plain(blocker, action, owner))
}
fn cask(blocker: &str, action: &str, owner: &str) -> String {
    if action.contains("homebrew-cask-check") || action.contains("Short Execution Memo") {
        return plain(blocker, action, owner);
    }
    format!("{}; start with cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb /absolute/path/to/release-notes.md after the public tap PR and install/uninstall proof exist; see docs/public-beta-operator-checklist.md `Short Execution Memo` for the fast path", plain(blocker, action, owner))
}
