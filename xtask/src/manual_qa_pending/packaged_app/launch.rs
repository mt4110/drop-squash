use std::path::Path;

pub(super) fn lines(artifact: Option<&str>, config_path: Option<&str>) -> Vec<String> {
    let Some(config_path) = config_path else {
        return Vec::new();
    };
    let Some(home) = home_dir(config_path) else {
        return Vec::new();
    };
    let Some(state) = state_dir(config_path) else {
        return Vec::new();
    };
    let mut lines = vec![
        format!("packaged-app launch home: {home}"),
        format!("packaged-app launch state dir: {state}"),
    ];
    if let Some(app) = artifact.and_then(app_bundle) {
        lines.push(format!(
            "packaged-app launch command: env DROP_SQUASH_HOME={} DROP_SQUASH_APP_STATE_DIR={} open -n -a {}",
            shell_quote(&home),
            shell_quote(&state),
            shell_quote(&app),
        ));
        lines.push(format!(
            "packaged-app drag event log launch command: env DROP_SQUASH_HOME={} DROP_SQUASH_APP_STATE_DIR={} DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-drag-events.jsonl open -n -a {}",
            shell_quote(&home),
            shell_quote(&state),
            shell_quote(&app),
        ));
    }
    lines
}

fn home_dir(config_path: &str) -> Option<String> {
    Path::new(config_path)
        .parent()?
        .parent()?
        .parent()?
        .parent()
        .map(|path| path.display().to_string())
}

fn app_bundle(artifact: &str) -> Option<String> {
    if artifact.ends_with(".app") {
        return Some(artifact.to_string());
    }
    let dmg = Path::new(artifact);
    let app = dmg.parent()?.parent()?.join("macos").join("DropSquash.app");
    app.exists().then(|| app.display().to_string())
}

fn state_dir(config_path: &str) -> Option<String> {
    Path::new(config_path)
        .parent()
        .map(|path| path.display().to_string())
}

fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::lines;

    #[test]
    fn prints_launch_commands_for_app_and_config_path() {
        let lines = lines(
            Some("/tmp/DropSquash.app"),
            Some("/tmp/state/Library/Application Support/DropSquash/config.json"),
        );

        assert!(lines
            .iter()
            .any(|line| line.contains("launch home: /tmp/state")));
        assert!(lines.iter().any(|line| line
            .contains("launch state dir: /tmp/state/Library/Application Support/DropSquash")));
        assert!(lines.iter().any(|line| line.contains("env DROP_SQUASH_HOME='/tmp/state' DROP_SQUASH_APP_STATE_DIR='/tmp/state/Library/Application Support/DropSquash' open -n -a '/tmp/DropSquash.app'")));
        assert!(lines.iter().any(
            |line| line.contains("DROP_SQUASH_MANUAL_QA_EVENT_LOG=/tmp/dsq-drag-events.jsonl")
        ));
        assert!(lines
            .iter()
            .any(|line| line.contains("open -n -a '/tmp/DropSquash.app'")));
    }

    #[test]
    fn resolves_sibling_app_for_dmg_artifact() {
        let dir = tempfile::tempdir().unwrap();
        let app = dir.path().join("macos/DropSquash.app");
        std::fs::create_dir_all(&app).unwrap();
        let dmg = dir.path().join("dmg/DropSquash.dmg");
        std::fs::create_dir_all(dmg.parent().unwrap()).unwrap();
        std::fs::write(&dmg, []).unwrap();

        let lines = lines(
            Some(dmg.to_str().unwrap()),
            Some("/tmp/state/Library/Application Support/DropSquash/config.json"),
        );

        assert!(lines.iter().any(|line| line.contains("open -n -a")));
    }
}
