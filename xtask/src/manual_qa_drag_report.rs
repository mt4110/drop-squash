const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-drag-report <output-dir> <history-path> [event-log-path]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (output_dir, history_path, event_log) = parse_args(args)?;
    print_status("output dir", &output_dir);
    print_recent_files(&output_dir)?;
    print_status("history path", &history_path);
    print_tail(&history_path)?;
    print_status("event log", &event_log);
    print_tail(&event_log)?;
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, String, String), String> {
    match args.as_slice() {
        [output_dir, history_path] => Ok((
            output_dir.clone(),
            history_path.clone(),
            "/tmp/dsq-drag-events.jsonl".to_string(),
        )),
        [output_dir, history_path, event_log] => {
            Ok((output_dir.clone(), history_path.clone(), event_log.clone()))
        }
        _ => Err(USAGE.to_string()),
    }
}

fn print_status(label: &str, path: &str) {
    let exists = std::path::Path::new(path).exists();
    println!(
        "{label}: {path} ({})",
        if exists { "exists" } else { "missing" }
    );
}

fn print_recent_files(output_dir: &str) -> Result<(), String> {
    println!("recent output files:");
    let mut entries = std::fs::read_dir(output_dir)
        .map_err(|error| format!("failed to read output dir {output_dir}: {error}"))?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.metadata().and_then(|m| m.modified()).ok());
    for entry in entries.into_iter().rev().take(5) {
        println!("- {}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

fn print_tail(path: &str) -> Result<(), String> {
    println!("tail:");
    match std::fs::read_to_string(path) {
        Ok(text) => {
            for line in text
                .lines()
                .rev()
                .take(5)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
            {
                println!("{line}");
            }
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("__MISSING__");
            Ok(())
        }
        Err(error) => Err(format!("failed to read {path}: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_args;

    #[test]
    fn uses_default_event_log_path() {
        let parsed = parse_args(vec!["/tmp/out".into(), "/tmp/history.jsonl".into()]).unwrap();
        assert_eq!(
            parsed,
            (
                "/tmp/out".into(),
                "/tmp/history.jsonl".into(),
                "/tmp/dsq-drag-events.jsonl".into()
            )
        );
    }

    #[test]
    fn accepts_explicit_event_log_path() {
        let parsed = parse_args(vec![
            "/tmp/out".into(),
            "/tmp/history.jsonl".into(),
            "/tmp/events.jsonl".into(),
        ])
        .unwrap();
        assert_eq!(parsed.2, "/tmp/events.jsonl");
    }
}
