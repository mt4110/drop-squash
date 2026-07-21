use super::extra_lines;

#[test]
fn reports_stash_and_link_commands_for_dmg_runs() {
    let directory = tempfile::tempdir().unwrap();
    let csv = directory.path().join("results.csv");
    std::fs::write(
        &csv,
        "backend,input,output\napple-native,/tmp/short.mov,/tmp/short.squashed.mp4\napple-native,/tmp/medium.mov,/tmp/medium.squashed.mp4\napple-native,/tmp/large.mov,/tmp/large.squashed.mp4\n",
    )
    .unwrap();
    let text = format!(
        "| App artifact | /tmp/DropSquash.dmg |\n| Output folder | /tmp/output |\n| Config path | /tmp/state/Library/Application Support/DropSquash/config.json |\n| History path | /tmp/state/Library/Application Support/DropSquash/history.jsonl |\n| Drag-and-drop conversion | Small `.mov` | Creates output |  |\n| Benchmark sample set | Three samples | CSV saved outside repo at {} |\n",
        csv.display()
    );

    let lines = extra_lines(&text);

    assert!(lines
        .iter()
        .any(|line| line.contains("fresh build command: CARGO_TARGET_DIR=/tmp/dsq-build-target pnpm --dir apps/desktop tauri build")));
    assert!(lines.iter().any(|line| line.contains(
        "fresh app artifact: /tmp/dsq-build-target/release/bundle/macos/DropSquash.app"
    )));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-installed-app status")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-installed-app stash")));
    assert!(lines.iter().any(|line| line.contains("launch home: /tmp")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-launch-app")));
    assert!(lines
        .iter()
        .any(|line| line.contains("fresh app launch helper command")));
    assert!(lines.iter().any(|line| line.contains("--open-panel")));
    assert!(lines
        .iter()
        .any(|line| line.contains("drag output inspect command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("drag history inspect command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-drag-report")));
    assert!(lines
        .iter()
        .any(|line| line.contains("panel sample dir: /tmp/dropsquash-qa-open-panel")));
    assert!(
        lines
            .iter()
            .any(|line| line
                .contains("panel sample open command: open /tmp/dropsquash-qa-open-panel"))
    );
    assert!(lines
        .iter()
        .any(|line| line.contains("drag result template")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-link-samples")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-open-chooser")));
    assert!(lines
        .iter()
        .any(|line| line.contains("manual-qa-drag-drop")));
    assert!(lines
        .iter()
        .any(|line| line.contains("synthetic drag is diagnostic only")));
    assert!(lines
        .iter()
        .any(|line| line.contains("/tmp/dsq-drag-events.jsonl")));
    assert!(lines
        .iter()
        .any(|line| line.contains("event log inspect command")));
    assert!(lines.iter().any(|line| line.contains("qa-not-smaller.mp4")));
    assert!(lines
        .iter()
        .any(|line| line.contains("panel sample open command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("open-file sample command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("open-file not-smaller command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("fresh open-file not-smaller command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("invalid sample command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("chooser not smaller sample command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mounted dmg relaunch note")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mounted dmg window probe command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mounted dmg pid count command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mounted dmg pid list command")));
    assert!(lines
        .iter()
        .any(|line| line.contains("mounted dmg pid note")));
    assert!(lines
        .iter()
        .any(|line| line.contains("visibility note: when the trial banner, license input, and install notice are visible together")));
}
