mod ax;

const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-drag-drop <panel-dir> <sample-name> [sample-name...]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (panel, samples) = parse_args(args)?;
    ensure_samples_exist(&panel, &samples)?;
    run_swift_helper(&panel, &samples)?;
    println!("manual QA drag and drop sent");
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, Vec<String>), String> {
    match args.as_slice() {
        [panel, samples @ ..] if !samples.is_empty() => Ok((panel.clone(), samples.to_vec())),
        _ => Err(USAGE.to_string()),
    }
}

fn ensure_samples_exist(panel: &str, samples: &[String]) -> Result<(), String> {
    for sample in samples {
        let path = std::path::Path::new(panel).join(sample);
        if !path.is_file() {
            return Err(format!("sample file does not exist: {}", path.display()));
        }
    }
    Ok(())
}

fn run_swift_helper(panel: &str, samples: &[String]) -> Result<(), String> {
    let script = ax::script(&normalize_tmp_path(panel), samples);
    let mut child = std::process::Command::new("swift")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to run swift helper: {error}"))?;
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| "swift helper stdin unavailable".to_string())?
        .write_all(script.as_bytes())
        .map_err(|error| format!("failed to write swift helper: {error}"))?;
    let status = child
        .wait()
        .map_err(|error| format!("failed to wait for swift helper: {error}"))?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| format!("swift helper exited with status {status}"))
}

fn normalize_tmp_path(path: &str) -> String {
    path.strip_prefix("/tmp/")
        .map(|suffix| format!("/private/tmp/{suffix}"))
        .unwrap_or_else(|| path.to_string())
}

#[cfg(test)]
mod tests {
    use super::{ensure_samples_exist, parse_args};

    #[test]
    fn parses_panel_and_samples() {
        let (panel, samples) = parse_args(vec![
            "/tmp/dropsquash-qa-open-panel".into(),
            "qa-small.mov".into(),
            "qa-medium.mov".into(),
        ])
        .unwrap();
        assert_eq!(panel, "/tmp/dropsquash-qa-open-panel");
        assert_eq!(samples, vec!["qa-small.mov", "qa-medium.mov"]);
    }

    #[test]
    fn rejects_missing_samples() {
        assert!(parse_args(vec!["/tmp/dropsquash-qa-open-panel".into()]).is_err());
    }

    #[test]
    fn rejects_missing_sample_files() {
        let dir = tempfile::tempdir().unwrap();

        let error = ensure_samples_exist(dir.path().to_str().unwrap(), &["qa-small.mov".into()])
            .unwrap_err();

        assert!(error.contains("sample file does not exist"));
    }
}
