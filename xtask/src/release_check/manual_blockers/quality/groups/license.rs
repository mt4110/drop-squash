use super::Groups;

pub(super) fn for_check(check: &str) -> Option<Groups> {
    match check {
        "Sandbox product setup" => Some(&[
            &["sandbox"],
            &["dropsquash"],
            &["intended product"],
            &["license keys enabled"],
        ]),
        "Sandbox purchase" => Some(&[
            &["sandbox"],
            &["intended product"],
            &["test buyer"],
            &["order"],
        ]),
        "Valid sandbox activation" => Some(&[
            &["lemon squeezy"],
            &["sandbox"],
            &["activating"],
            &["disabled"],
            &["cache", "license.json"],
            &["pro"],
            &["fingerprint"],
            &["instance"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
        ]),
        "Empty key activation" => Some(&[
            &["activate"],
            &["disabled"],
            &["cache", "license.json"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
        ]),
        "Invalid key activation" => Some(&[
            &["activating"],
            &["disabled"],
            &["cache", "license.json"],
            &["friendly"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
        ]),
        "License network failure" => Some(&[
            &["cache", "license.json"],
            &["friendly"],
            &["network"],
            &["existing"],
            &["valid"],
            &["preserved", "intact"],
            &["fingerprint"],
            &["instance"],
            &["raw key"],
            &["absent", "no raw key", "without raw key"],
        ]),
        "Forget license on this Mac" => Some(&[
            &["forgetting"],
            &["disabled"],
            &["cache", "license.json"],
            &["removed", "cleared", "deleted", "clears"],
            &["trial", "locked"],
        ]),
        _ => None,
    }
}
