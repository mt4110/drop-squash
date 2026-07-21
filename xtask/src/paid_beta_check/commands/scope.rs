pub(super) fn packaged(required: &[String], pending: &impl Fn(&str) -> bool) -> bool {
    required
        .iter()
        .any(|name| name == "Packaged macOS manual QA" && pending(name))
}

pub(super) fn license(required: &[String], pending: &impl Fn(&str) -> bool) -> bool {
    required.iter().any(|name| {
        matches!(
            name.as_str(),
            "Lemon Squeezy product setup"
                | "Lemon Squeezy sandbox purchase"
                | "Valid sandbox activation"
        ) && pending(name)
    })
}

pub(super) fn distribution(required: &[String], pending: &impl Fn(&str) -> bool) -> bool {
    required.iter().any(|name| {
        matches!(
            name.as_str(),
            "Signed DMG" | "Notarized and stapled DMG" | "Gatekeeper clean-machine open"
        ) && pending(name)
    })
}
