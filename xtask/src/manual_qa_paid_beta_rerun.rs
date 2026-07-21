use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

const DEFAULT_MANUAL: &str = "docs/manual-qa.md";
const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-paid-beta-rerun [manual-qa.md]";
const PAID_BETA_CHECKLIST: &str = "docs/paid-beta-operator-checklist.md";
const PACKAGED_GUIDE: &str = "docs/manual-qa.md";
const LICENSE_RUNBOOK: &str = "docs/license-sandbox-runbook.md";
const MANUAL_LICENSE_ISSUANCE: &str = "docs/manual-beta-license-issuance.md";
const SIGNED_DMG_RUNBOOK: &str = "docs/signed-dmg-runbook.md";
const DISTRIBUTION_HANDOFF: &str =
    "scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";
const PUBLIC_WEB_TRACK: &str =
    "cargo run -p xtask -- productization-status --track 'Public web proof'";
const PUBLIC_BETA_CHECKLIST: &str = "docs/public-beta-operator-checklist.md";
const PUBLIC_WEB_FIRST_PASS: &str = "cargo run -p xtask -- public-web-ready";
const PUBLIC_WEB_OPERATOR_MEMO: &str = "cargo run -p xtask -- public-web-rerun";
const LICENSE_BROWSER_SIGN_IN_CHECKPOINT: &str =
    "paid beta license browser sign-in checkpoint: if Lemon Squeezy still shows `Sign in to Lemon Squeezy` or `auth.lemonsqueezy.com/login`, stop and sign in before recording sandbox product setup, purchase, or activation";
const LICENSE_BROWSER_SIGN_IN_SUCCESS: &str =
    "paid beta license browser sign-in success: continue only after the Lemon Squeezy dashboard is open and sandbox mode is visible for the intended DropSquash product";
const LICENSE_ACTIVATION_LOOP: &str =
    "paid beta license activation loop: run sandbox quickstart 2 before the UI action, then sandbox quickstart 4 after Pro appears so the cache delta is recorded";
const DISTRIBUTION_TARGET_HINT: &str =
    "paid beta distribution isolated target hint: if xtask verification stalls in the everyday target tree, rerun distribution helpers with CARGO_TARGET_DIR=/tmp/dsq-xtask-target";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let path = parse_args(args)?;
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read manual QA file: {error}"))?;
    crate::manual_qa_dirty_worktree::print_quickstart();
    for line in crate::manual_qa_license_rerun::status::lines(&path)? {
        println!("paid beta {line}");
    }
    for line in crate::manual_qa_distribution_rerun::status::lines() {
        println!("paid beta {line}");
    }
    if has_benchmark_csv(&text) {
        println!(
            "paid beta packaged rerun: cargo run -p xtask -- manual-qa-packaged-rerun '{}'",
            quote(&path)
        );
        println!(
            "paid beta deterministic helper: prepare a fresh draft and checked benchmark CSV, then run `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>` as the standard paid-beta pass before filling sandbox or signing rows; keep `manual-qa-packaged-rerun` for the packaged-app slice"
        );
        println!(
            "{}",
            crate::manual_qa_observation::packaged_visibility_reminder(
                "paid beta packaged observation reminder",
            )
        );
    } else {
        println!("paid beta packaged rerun blocked: manual QA file is missing a checked Benchmark sample set CSV outside the repository");
        println!(
            "paid beta recovery order: rerun `manual-qa-prepare --reset-trial`, `benchmark --release-set`, `benchmark-csv-check`, then `manual-qa-ready-all` before packaged, license, or distribution reruns"
        );
    }
    for line in next_lines(&path) {
        println!("{line}");
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<PathBuf, String> {
    match args.as_slice() {
        [] => Ok(PathBuf::from(DEFAULT_MANUAL)),
        [path] if path != "--help" && path != "-h" => Ok(PathBuf::from(path)),
        _ => Err(USAGE.to_string()),
    }
}

fn has_benchmark_csv(text: &str) -> bool {
    text.lines()
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect::<Vec<_>>()
        })
        .find(|cells| cells.len() >= 3 && cells[0] == "Benchmark sample set")
        .and_then(|cells| crate::csv_evidence::existing_outside_repo_path(&cells[2]))
        .is_some()
}

fn quote(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}

fn next_lines(path: &Path) -> [String; 20] {
    let manual = quote(path);
    [
        format!("packaged manual QA guide: {PACKAGED_GUIDE}"),
        format!("next license sandbox runbook: {LICENSE_RUNBOOK} (see `Short Execution Memo` for the fast path)"),
        format!("manual beta license issuance: {MANUAL_LICENSE_ISSUANCE}"),
        format!("next signed DMG runbook: {SIGNED_DMG_RUNBOOK} (see `Short Execution Memo` for the fast path)"),
        format!("paid beta distribution snapshot handoff: {DISTRIBUTION_HANDOFF}"),
        LICENSE_BROWSER_SIGN_IN_CHECKPOINT.to_string(),
        LICENSE_BROWSER_SIGN_IN_SUCCESS.to_string(),
        format!("paid beta license rerun: cargo run -p xtask -- manual-qa-license-rerun '{manual}'"),
        LICENSE_ACTIVATION_LOOP.to_string(),
        DISTRIBUTION_TARGET_HINT.to_string(),
        format!("paid beta distribution rerun: cargo run -p xtask -- manual-qa-distribution-rerun '{manual}'"),
        format!("paid beta license markdown rows: cargo run -p xtask -- manual-qa-pending '{manual}' --section license | rg '^license (product setup|sandbox purchase|valid activation) markdown row:'"),
        format!("paid beta distribution markdown rows: cargo run -p xtask -- manual-qa-pending '{manual}' --section distribution | rg 'distribution .*markdown row:'"),
        format!("paid beta local-proof gate: cargo run -p xtask -- manual-qa-check '{manual}' --section local-proof"),
        format!("paid beta license gate: cargo run -p xtask -- manual-qa-check '{manual}' --section license"),
        format!("paid beta distribution gate: cargo run -p xtask -- manual-qa-check '{manual}' --section distribution"),
        format!("paid beta final gate: cargo run -p xtask -- manual-qa-check '{manual}'"),
        "paid beta blocker gate: cargo run -p xtask -- paid-beta-check".to_string(),
        format!("paid beta operator checklist: {PAID_BETA_CHECKLIST}"),
        format!(
            "after paid-beta technical proof: start with `{PUBLIC_WEB_FIRST_PASS}`, use `{PUBLIC_WEB_OPERATOR_MEMO}` as the operator memo when needed, then run `{PUBLIC_WEB_TRACK}` and follow `{PUBLIC_BETA_CHECKLIST}` (`Short Execution Memo` for the fast path) before Stripe or Lemon Squeezy production onboarding"
        ),
    ]
}
