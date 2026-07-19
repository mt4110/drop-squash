const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-secure-share-report <receipt.json> [event-log.jsonl]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (receipt_path, event_log_path) = parse_args(args)?;
    let receipt = read_json(&receipt_path)?;
    println!("receipt: {receipt_path}");
    print_field(&receipt, "input_name");
    print_field(&receipt, "output_name");
    print_field(&receipt, "mask_mode");
    print_field(&receipt, "mask_rect_count");
    print_field(&receipt, "output_sha256");
    println!("event log: {event_log_path}");
    match std::fs::read_to_string(&event_log_path) {
        Ok(text) => {
            let lines = text.lines().collect::<Vec<_>>();
            println!("event count: {}", lines.len());
            for kind in [
                "secure-share-toggle",
                "secure-share-mode",
                "secure-share-rect-add",
                "secure-share-result",
                "receipt-open",
            ] {
                println!("{kind}: {}", contains_kind(&lines, kind));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("event log missing");
        }
        Err(error) => {
            return Err(format!(
                "failed to read event log {event_log_path}: {error}"
            ))
        }
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, String), String> {
    match args.as_slice() {
        [receipt_path] => Ok((
            receipt_path.clone(),
            "/tmp/dsq-p2-alpha-multirect-events.jsonl".to_string(),
        )),
        [receipt_path, event_log_path] => Ok((receipt_path.clone(), event_log_path.clone())),
        _ => Err(USAGE.to_string()),
    }
}

fn read_json(path: &str) -> Result<serde_json::Value, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("failed to read {path}: {error}"))?;
    serde_json::from_str(&text).map_err(|error| format!("failed to parse {path}: {error}"))
}

fn print_field(receipt: &serde_json::Value, name: &str) {
    println!(
        "{name}: {}",
        receipt.get(name).unwrap_or(&serde_json::Value::Null)
    );
}

fn contains_kind(lines: &[&str], kind: &str) -> bool {
    lines.iter().any(|line| {
        serde_json::from_str::<serde_json::Value>(line)
            .ok()
            .and_then(|value| {
                value
                    .get("kind")
                    .and_then(serde_json::Value::as_str)
                    .map(|it| it == kind)
            })
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::{contains_kind, parse_args};

    #[test]
    fn uses_default_event_log_path() {
        let parsed = parse_args(vec!["/tmp/receipt.json".into()]).unwrap();
        assert_eq!(parsed.1, "/tmp/dsq-p2-alpha-multirect-events.jsonl");
    }

    #[test]
    fn matches_event_kind() {
        let lines =
            [r#"{"kind":"secure-share-result","detail":"take0.squashed.secure-share.json"}"#];
        assert!(contains_kind(&lines, "secure-share-result"));
        assert!(!contains_kind(&lines, "receipt-open"));
    }
}
