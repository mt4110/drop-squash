pub(super) fn lines(rows: &[(String, String)]) -> Vec<String> {
    let mut lines = Vec::new();
    if has_label(rows, "Sandbox product setup") || has_label(rows, "Sandbox purchase") {
        lines.push("license sandbox runbook: docs/license-sandbox-runbook.md".to_string());
    }
    if has_label(rows, "Sandbox product setup")
        || has_label(rows, "Sandbox purchase")
        || has_label(rows, "Valid sandbox activation")
    {
        lines.push("license sandbox next step 1: fill Sandbox product setup first".to_string());
        lines.push("license sandbox next step 2: complete Sandbox purchase with a concrete test buyer order id or order number".to_string());
        lines.push(
            "license sandbox next step 3: run Valid sandbox activation and inspect the cache"
                .to_string(),
        );
    }
    if has_label(rows, "Sandbox product setup") {
        lines.push("license setup reminder: confirm the intended product is DropSquash, sandbox mode is active, license keys are enabled, and private store IDs are not recorded before filling the row".to_string());
    }
    if has_label(rows, "Sandbox purchase") {
        lines.push("license purchase reminder: complete one sandbox checkout for the intended DropSquash product, record the test buyer, and keep the concrete order id or order number outside secret values".to_string());
    }
    if has_label(rows, "Valid sandbox activation") {
        lines.push("license valid activation reminder: use a real sandbox key outside this repository, confirm submit stays disabled while Activating, then record Pro plus cache evidence with fingerprint present 64-character lowercase hex and instance_id present".to_string());
    }
    lines
}

pub(super) fn needs_status_reminder(rows: &[(String, String)]) -> bool {
    rows.iter().any(|(label, _)| {
        matches!(
            label.as_str(),
            "Empty key activation"
                | "Invalid key activation"
                | "Valid sandbox activation"
                | "License network failure"
                | "Expired license refresh"
                | "`cargo run -p dropsquash -- license status`"
        )
    })
}

pub(super) fn has_forget_row(rows: &[(String, String)]) -> bool {
    rows.iter()
        .any(|(label, _)| label == "Forget license on this Mac")
}

fn has_label(rows: &[(String, String)], expected: &str) -> bool {
    rows.iter().any(|(label, _)| label == expected)
}

#[cfg(test)]
mod tests {
    use super::lines;

    #[test]
    fn prints_setup_purchase_and_activation_reminders() {
        let rows = vec![
            ("Sandbox product setup".to_string(), String::new()),
            ("Sandbox purchase".to_string(), String::new()),
            ("Valid sandbox activation".to_string(), String::new()),
        ];
        let lines = lines(&rows);

        assert!(lines
            .iter()
            .any(|line| line.contains("license sandbox runbook")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license sandbox next step 1")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license sandbox next step 2")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license sandbox next step 3")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license setup reminder")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license purchase reminder")));
        assert!(lines
            .iter()
            .any(|line| line.contains("license valid activation reminder")));
    }
}
