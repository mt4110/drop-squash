use super::{next_lines, run, state_lines};

#[test]
fn rejects_extra_arguments() {
    let error = run(vec!["extra".into()]).unwrap_err();
    assert!(error.contains("public-web-rerun"));
}

#[test]
fn prints_expected_public_web_lines() {
    run(vec![]).unwrap();
}

#[test]
fn exposes_public_web_fast_path() {
    let lines = next_lines();

    assert!(lines.iter().any(|line| line.contains("public-web-ready")));
    assert!(lines
        .iter()
        .any(|line| line.contains("public-web-handoff.sh")));
    assert!(lines.iter().any(|line| line.contains("website-check")));
    assert!(lines
        .iter()
        .any(|line| line.contains("npm --prefix apps/site run verify:site")));
    assert!(lines.iter().any(|line| line.contains("public-web-probe")));
    assert!(lines
        .iter()
        .any(|line| line.contains("productization-status --track 'Public web proof'")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/public-beta-operator-checklist.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("docs/website-deployment-runbook.md")));
    assert!(lines
        .iter()
        .any(|line| line.contains("public web next step 1")));
    assert!(lines
        .iter()
        .any(|line| line.contains("https://dropsquash.app/release-status")));
    assert!(lines
        .iter()
        .any(|line| line.contains("https://dropsquash.app/pricing")));
    assert!(lines
        .iter()
        .any(|line| line.contains("https://dropsquash.app/refund")));
    assert!(lines
        .iter()
        .any(|line| line.contains("https://store.lemonsqueezy.com/checkout/buy/<id>")));
    assert!(lines
        .iter()
        .any(|line| line.contains("dig +short A dropsquash.app")));
    assert!(lines.iter().any(|line| line.contains("162.159.143.30")));
    assert!(lines.iter().any(|line| line.contains("172.66.3.26")));
    assert!(lines
        .iter()
        .any(|line| line.contains("_openai-site-verification.dropsquash.app")));
    assert!(lines
        .iter()
        .any(|line| line.contains("_cf-custom-hostname.dropsquash.app")));
    assert!(lines
        .iter()
        .any(|line| line.contains("stale pricing/refund copy")));
    assert!(lines
        .iter()
        .any(|line| line.contains("dropsquash-app.system-obj-gg.chatgpt.site")));
    assert!(lines.iter().any(|line| line.contains("provenance")));
    assert!(lines.iter().any(|line| line.contains("draft-era copy")));
    assert!(lines
        .iter()
        .any(|line| line.contains("intentional committed site source")));
    assert!(lines.iter().any(|line| line.contains("publish-check")));
}

#[test]
fn includes_dirty_worktree_guidance_when_repo_is_dirty() {
    let source = std::fs::read_to_string(format!(
        "{}/src/public_web_rerun.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    assert!(source.contains("manual_qa_dirty_worktree::quickstart_lines"));
    assert!(source.contains("save/deploy only from an intentional committed site source"));
}

#[test]
fn reports_source_head_and_dirty_paths() {
    let lines = state_lines(
        Some("abc123"),
        Some(" M docs/manual-qa.md\n M docs/release.md\n"),
    );

    assert!(lines
        .iter()
        .any(|line| line.contains("public web source head: abc123")));
    assert!(lines
        .iter()
        .any(|line| line.contains("public web dirty paths:  M docs/manual-qa.md")));
}
