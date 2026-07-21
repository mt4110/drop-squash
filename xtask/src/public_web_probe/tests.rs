use super::{blocker_state, canonical_status, owner_only_status, summarize};

#[test]
fn summarize_uses_first_line() {
    assert_eq!(summarize("HTTP/2 401 \nserver: cloudflare\n"), "HTTP/2 401");
}

#[test]
fn summarize_handles_empty_output() {
    assert_eq!(summarize(" \n"), "no output");
}

#[test]
fn summarize_skips_dig_warnings() {
    assert_eq!(
        summarize(";; Warning: parser noise\n172.66.3.26\n"),
        "172.66.3.26"
    );
}

#[test]
fn summarize_skips_host_server_lines() {
    assert_eq!(
        summarize("Server:\t\t127.0.0.1\nHost dropsquash.app not found: 3(NXDOMAIN)\n"),
        "Host dropsquash.app not found: 3(NXDOMAIN)"
    );
}

#[test]
fn source_keeps_expected_targets() {
    let source = std::fs::read_to_string(format!(
        "{}/src/public_web_probe.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    assert!(source.contains("current_date::display"));
    assert!(source.contains("dropsquash-app.system-obj-gg.chatgpt.site/release-status"));
    assert!(source.contains("https://dropsquash.app/release-status"));
    assert!(source.contains("https://dropsquash.app/pricing"));
    assert!(source.contains("https://dropsquash.app/refund"));
    assert!(source.contains("dropsquash.app\", \"A"));
    assert!(source.contains("TXT"));
    assert!(source.contains("probe_dns"));
    assert!(source.contains("\"host\""));
    assert!(source.contains("curl"));
    assert!(source.contains("162.159.143.30 172.66.3.26"));
    assert!(source.contains("openai-site-verification=1B5z4jz2Z2ifY2MsN3UggSHPMaSsaGwrOgttmLnIw6g"));
    assert!(source.contains("c70e75c8-8887-4b4a-a390-72fd4a8400fd"));
    assert!(source.contains("expected domain status"));
    assert!(source.contains("owner-only status"));
    assert!(source.contains("canonical status"));
    assert!(source.contains("blocker state"));
}

#[test]
fn owner_only_status_recognizes_live_owner_only_response() {
    assert_eq!(owner_only_status("HTTP/2 401\n"), "owner-only deploy alive");
}

#[test]
fn owner_only_status_rechecks_non_401_response() {
    assert_eq!(
        owner_only_status("HTTP/2 200\n"),
        "recheck owner-only deploy manually"
    );
}

#[test]
fn canonical_status_marks_unresolved_host_as_not_ready() {
    assert_eq!(
        canonical_status(
            "Host dropsquash.app not found: 3(NXDOMAIN)\n",
            &["curl: (6) Could not resolve host: dropsquash.app\n"]
        ),
        "canonical host not ready"
    );
}

#[test]
fn blocker_state_stays_open_while_canonical_host_is_not_ready() {
    assert_eq!(
        blocker_state(
            "Host dropsquash.app not found: 3(NXDOMAIN)\n",
            &["curl: (6) Could not resolve host: dropsquash.app\n"]
        ),
        "keep public-web blockers open"
    );
}
