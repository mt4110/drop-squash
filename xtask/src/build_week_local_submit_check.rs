mod macos;

use sha2::{Digest, Sha256};
use std::path::Path;

const DMG: &str = "target/release/bundle/dmg/DropSquash_0.1.0_aarch64.dmg";
const DEMO_URL: &str = "https://youtu.be/6ljSp4eoq0g";
const FIELDS: &str = "docs/build-week-devpost-fields.md";
const ENTRY: &str = "docs/build-week-final-entry-sheet.md";
const PACKET: &str = "docs/build-week-submit-packet.md";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    if !args.is_empty() {
        return Err("usage: cargo run -p xtask -- build-week-local-submit-check".into());
    }
    crate::build_week_phase5_check::run(Vec::new())?;
    let packet = read_text(PACKET)?;
    let expected_sha = expected_dmg_sha(&packet)?;
    check_dmg(&expected_sha)?;
    macos::verify_distribution(Path::new(DMG))?;
    check_text(&read_text(FIELDS)?, &fields_required(&expected_sha), FIELDS)?;
    check_text(&read_text(ENTRY)?, &entry_required(&expected_sha), ENTRY)?;
    check_text(&packet, &packet_required(), PACKET)?;
    println!("Build Week local submit checks passed");
    println!("External still manual: /feedback session ID, Devpost submit");
    Ok(())
}

fn check_dmg(expected_sha: &str) -> Result<(), String> {
    let bytes = std::fs::read(DMG).map_err(|error| format!("Build Week DMG: {error}"))?;
    let digest = sha256_hex(&bytes);
    if digest == expected_sha {
        return Ok(());
    }
    Err(format!("Build Week DMG SHA-256 mismatch: {digest}"))
}

fn read_text(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))
}

fn expected_dmg_sha(text: &str) -> Result<String, String> {
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        if line.trim() == "- SHA-256:" {
            return lines
                .next()
                .and_then(|line| line.trim().strip_prefix('`'))
                .and_then(|line| line.strip_suffix('`'))
                .map(str::to_string)
                .ok_or_else(|| format!("{PACKET}: malformed SHA-256 value"));
        }
    }
    Err(format!("{PACKET}: missing SHA-256 field"))
}

fn check_text(text: &str, required: &[String], path: &str) -> Result<(), String> {
    for needle in required {
        if !text.contains(needle) {
            return Err(format!("{path}: missing {needle:?}"));
        }
    }
    Ok(())
}

fn fields_required(expected_sha: &str) -> Vec<String> {
    [
        "DropSquash Secure Share Alpha".to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
        DEMO_URL.to_string(),
        "PASTE_CODEX_FEEDBACK_SESSION_ID_HERE".to_string(),
        "ScreenCaptureKit".to_string(),
        "Ed25519".to_string(),
        expected_sha.to_string(),
    ]
    .into()
}

fn entry_required(expected_sha: &str) -> Vec<String> {
    [
        DEMO_URL.to_string(),
        "PASTE_CODEX_FEEDBACK_SESSION_ID_HERE".to_string(),
        "https://github.com/mt4110/drop-squash".to_string(),
        "DropSquash Secure Share Alpha".to_string(),
        "ScreenCaptureKit".to_string(),
        "Ed25519".to_string(),
        expected_sha.to_string(),
    ]
    .into()
}

fn packet_required() -> Vec<String> {
    [
        "Developer Tools".to_string(),
        "DropSquash Secure Share Alpha".to_string(),
        "devposttesting".to_string(),
        "build-week-event@openai.com".to_string(),
        "42774ba8-fe01-4439-99d1-6c8565e499aa".to_string(),
    ]
    .into()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
