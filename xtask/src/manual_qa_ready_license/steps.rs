pub(super) const DRAFT_MARKER: &str = "Prepared manual QA draft only.";
pub(super) const FRESH_APP: &str = "/tmp/dsq-build-target/release/bundle/macos/DropSquash.app";
pub(super) const ISOLATED_PREPARE: &str = "cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash\" --state-dir /tmp/dropsquash-manual-qa-state --output-dir /tmp/dropsquash-manual-qa-output --markdown-output /tmp/dropsquash-manual-qa-prepared.md";
pub(super) const PAID_BETA_CHECKLIST: &str = "docs/paid-beta-operator-checklist.md";
pub(super) const PACKAGED_GUIDE: &str = "docs/manual-qa.md";
pub(super) const SIGNED_DMG_RUNBOOK: &str = "docs/signed-dmg-runbook.md";
pub(super) const MANUAL_LICENSE_ISSUANCE: &str = "docs/manual-beta-license-issuance.md";
pub(super) const DISTRIBUTION_HANDOFF: &str =
    "distribution snapshot handoff: scripts/manual-qa-distribution-handoff.sh /tmp/dropsquash-qa-snapshot-$(git rev-parse --short HEAD)";
pub(super) const BROWSER_SIGN_IN_HINT: &str =
    "license browser sign-in checkpoint: if Lemon Squeezy still shows `Sign in to Lemon Squeezy` or `auth.lemonsqueezy.com/login`, stop and sign in before recording sandbox product setup, purchase, or activation";
pub(super) const BROWSER_SIGN_IN_SUCCESS: &str =
    "license browser sign-in success: continue only after the Lemon Squeezy dashboard is open and sandbox mode is visible for the intended DropSquash product";
pub(super) const READY_ALL_HINT: &str =
    "license deterministic helper: if you have a fresh prepared manual-QA draft and checked benchmark CSV, run `cargo run -p xtask -- manual-qa-ready-all <manual-qa.md> <results.csv>` before this rerun";
pub(super) const PUBLIC_WEB_HANDOFF: &str =
    "after license proof: start with `cargo run -p xtask -- public-web-ready`, use `cargo run -p xtask -- public-web-rerun` as the operator memo when needed, then run `cargo run -p xtask -- productization-status --track 'Public web proof'` and follow `docs/public-beta-operator-checklist.md` (`Short Execution Memo` for the fast path) before production onboarding";

const FRESH_BUILD: &str =
    "CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build";

pub(super) fn print_intro() {
    println!("next license sandbox runbook: docs/license-sandbox-runbook.md");
    println!("manual beta license issuance: {MANUAL_LICENSE_ISSUANCE}");
    println!("license operator checklist: {PAID_BETA_CHECKLIST}");
    println!("packaged manual QA guide: {PACKAGED_GUIDE}");
    println!("signed DMG runbook: {SIGNED_DMG_RUNBOOK}");
    println!("{DISTRIBUTION_HANDOFF}");
    println!("{BROWSER_SIGN_IN_HINT}");
    println!("{BROWSER_SIGN_IN_SUCCESS}");
    println!("{READY_ALL_HINT}");
    println!("fresh packaged-app build command: {FRESH_BUILD}");
    println!("fresh packaged-app artifact: {FRESH_APP}");
    println!("copy-ready sandbox markdown rows: use `next sandbox markdown rows command` after sign-in and after the app-side activation loop");
}

pub(super) fn print_next_steps() {
    println!("next license sandbox step 1: fill Sandbox product setup");
    println!(
        "next license sandbox step 2: complete Sandbox purchase with a concrete test buyer order id or order number"
    );
    println!("next license sandbox step 3: run Valid sandbox activation and inspect the cache");
    println!("next license activation loop: run sandbox quickstart 2 before the UI action, then sandbox quickstart 4 after Pro appears so the cache delta is recorded");
    println!("{PUBLIC_WEB_HANDOFF}");
}
