pub(super) fn print_plan() {
    println!("manual QA License diagnostics command:");
    println!("{}", diagnostics_command());
    println!("manual QA License Sandbox observation rows:");
    for row in rows() {
        println!("{row}");
    }
}

pub(super) fn rows() -> Vec<String> {
    vec![
        "| Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded |  |".to_string(),
        "| Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order |  |".to_string(),
        "| Empty key activation | Empty key leaves Activate disabled; checked license cache has no raw key, no fingerprint, and no instance |  |".to_string(),
        "| Invalid key activation | Activating state disables submit; friendly license error; inspected license cache has no raw key, no fingerprint, and no instance |  |".to_string(),
        "| Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key |  |".to_string(),
        "| License network failure | Friendly network error; checked existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key |  |".to_string(),
        "| Expired license refresh | Attempted conversion with expired offline grace cache shows reconnect prompt; conversion is blocked before starting; checked license cache has no raw key |  |".to_string(),
        "| Forget license on this Mac | Forgetting state disables action; confirmed license cache removed; observed app returns to trial or locked state |  |".to_string(),
        format!(
            "| {} | Record `raw license key persisted`, `license cache fingerprint`, and `license cache instance_id` without pasting the sandbox key |  |",
            diagnostics_command()
        ),
    ]
}

fn diagnostics_command() -> &'static str {
    "`cargo run -p dropsquash -- license status`"
}

#[cfg(test)]
mod tests {
    use super::{diagnostics_command, rows};

    #[test]
    fn generated_rows_match_required_manual_qa_checks() {
        let rows = rows().join("\n");

        for check in [
            "Sandbox product setup",
            "Sandbox purchase",
            "Empty key activation",
            "Invalid key activation",
            "Valid sandbox activation",
            "License network failure",
            "Expired license refresh",
            "Forget license on this Mac",
        ] {
            assert!(crate::manual_qa_check::requirements::REQUIRED_CHECKS.contains(&check));
            assert!(rows.contains(check));
        }
    }

    #[test]
    fn generated_rows_exist_in_manual_qa_template() {
        let template = std::fs::read_to_string("../docs/manual-qa.md").unwrap();

        for row in rows() {
            let check = row.split('|').nth(1).unwrap().trim();
            if check == diagnostics_command() {
                continue;
            }
            assert!(template.contains(check));
        }
    }

    #[test]
    fn diagnostics_row_names_safe_evidence_lines() {
        let rows = rows().join("\n");

        assert!(rows.contains("raw license key persisted"));
        assert!(rows.contains("license cache fingerprint"));
        assert!(rows.contains("license cache instance_id"));
        assert!(rows.contains("without pasting the sandbox key"));
    }
}
