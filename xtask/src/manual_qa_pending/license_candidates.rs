pub(super) fn lines() -> [&'static str; 14] {
    [
        "license product setup row candidate: Intended product was DropSquash; Lemon Squeezy sandbox mode was active; sandbox license keys were enabled; confirmed private store IDs were not recorded",
        "license product setup markdown row: | Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded | Intended product was DropSquash; Lemon Squeezy sandbox mode was active; sandbox license keys were enabled; confirmed private store IDs were not recorded |",
        "license sandbox purchase row candidate: Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded the sandbox order id or order number without private store IDs",
        "license sandbox purchase markdown row: | Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order | Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded the sandbox order id or order number without private store IDs |",
        "license empty activation row candidate: Empty key left Activate disabled; checked license cache diagnostics showed raw license key persisted: no, fingerprint missing, and instance_id missing",
        "license empty activation markdown row: | Empty key activation | Empty key leaves Activate disabled; checked license cache has no raw key, no fingerprint, and no instance | Empty key left Activate disabled; checked license cache diagnostics showed raw license key persisted: no, fingerprint missing, and instance_id missing |",
        "license invalid activation row candidate: Activating state disabled submit; friendly license error; checked license cache diagnostics showed raw license key persisted: no, fingerprint missing, and instance_id missing",
        "license invalid activation markdown row: | Invalid key activation | Activating state disables submit; friendly license error; inspected license cache has no raw key, no fingerprint, and no instance | Activating state disabled submit; friendly license error; checked license cache diagnostics showed raw license key persisted: no, fingerprint missing, and instance_id missing |",
        "license valid activation row candidate: Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present",
        "license valid activation markdown row: | Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key | Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present |",
        "license network failure row candidate: Friendly network error appeared; checked existing valid license cache diagnostics still showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present",
        "license network failure markdown row: | License network failure | Friendly network error; checked existing valid cache with 64-character lowercase hex fingerprint and `instance_id` fields remains intact and has no raw key | Friendly network error appeared; checked existing valid license cache diagnostics still showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present |",
        "license expired refresh row candidate: Attempted conversion with expired offline grace cache showed reconnect prompt; conversion was blocked before starting; checked license cache diagnostics showed raw license key persisted: no",
        "license expired refresh markdown row: | Expired license refresh | Attempted conversion with expired offline grace cache shows reconnect prompt; conversion is blocked before starting; checked license cache has no raw key | Attempted conversion with expired offline grace cache showed reconnect prompt; conversion was blocked before starting; checked license cache diagnostics showed raw license key persisted: no |",
    ]
}

pub(crate) fn sandbox_lines() -> [&'static str; 6] {
    [
        "license product setup row candidate: Intended product was DropSquash; Lemon Squeezy sandbox mode was active; sandbox license keys were enabled; confirmed private store IDs were not recorded",
        "license product setup markdown row: | Sandbox product setup | Intended product is DropSquash, sandbox license keys are enabled, and private store IDs are not recorded | Intended product was DropSquash; Lemon Squeezy sandbox mode was active; sandbox license keys were enabled; confirmed private store IDs were not recorded |",
        "license sandbox purchase row candidate: Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded the sandbox order id or order number without private store IDs",
        "license sandbox purchase markdown row: | Sandbox purchase | Sandbox checkout completes for the intended product and test buyer order | Sandbox checkout completed for the intended DropSquash product and test buyer order; recorded the sandbox order id or order number without private store IDs |",
        "license valid activation row candidate: Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present",
        "license valid activation markdown row: | Valid sandbox activation | Lemon Squeezy sandbox activation request disables submit while Activating; Pro state; checked license cache has 64-character lowercase hex fingerprint and `instance_id` fields with no raw key | Lemon Squeezy sandbox activation request disabled submit while Activating; Pro state; checked license cache diagnostics showed raw license key persisted: no, fingerprint present 64-character lowercase hex, and instance_id present |",
    ]
}

#[cfg(test)]
mod tests {
    use super::sandbox_lines;

    #[test]
    fn sandbox_lines_only_cover_paid_beta_rows() {
        let lines = sandbox_lines();
        assert_eq!(lines.len(), 6);
        assert!(lines
            .iter()
            .any(|line| line.contains("Sandbox product setup")));
        assert!(lines.iter().any(|line| line.contains("Sandbox purchase")));
        assert!(lines
            .iter()
            .any(|line| line.contains("Valid sandbox activation")));
        assert!(!lines
            .iter()
            .any(|line| line.contains("Invalid key activation")));
    }
}
