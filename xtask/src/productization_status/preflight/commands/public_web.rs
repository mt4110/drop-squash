pub(super) fn lines() -> Vec<String> {
    let today = crate::current_date::display();
    vec![
        "preflight combined public-web ready: cargo run -p xtask -- public-web-ready".to_string(),
        "preflight optional public-web rerun: cargo run -p xtask -- public-web-rerun".to_string(),
        "preflight public web gate: cargo run -p xtask -- website-check".to_string(),
        "preflight deployable site gate: npm --prefix apps/site run verify:site".to_string(),
        "preflight public web probe: cargo run -p xtask -- public-web-probe".to_string(),
        "preflight dns probe: dig +short A dropsquash.app; dig +short TXT _openai-site-verification.dropsquash.app; dig +short TXT _cf-custom-hostname.dropsquash.app".to_string(),
        "preflight canonical host state: confirm canonical / pricing / refund now return HTTP 200 before moving public-web blockers".to_string(),
        "preflight owner-history note: dropsquash-app.system-obj-gg.chatgpt.site is provenance only; canonical blocker evidence must come from dropsquash.app".to_string(),
        "preflight canonical content note: pricing and refund are now aligned on production; only reopen those blockers if canonical copy regresses".to_string(),
        format!(
            "preflight dns evidence: trust cargo run -p xtask -- public-web-probe for the current resolver result on {today}"
        ),
        "preflight dns fallback: if dig prints no answer, the probe falls back to host so resolver failures still stay explicit".to_string(),
        "preflight blocker rule: keep Live checkout link blocked until the canonical pricing page opens the intended live checkout URL; pricing/refund stay verified unless production copy regresses".to_string(),
        "preflight owner-history rule: do not use the owner-history host as pricing/refund blocker evidence".to_string(),
        "preflight public beta checklist: docs/public-beta-operator-checklist.md".to_string(),
        "preflight production URLs: verify https://dropsquash.app/release-status https://dropsquash.app/pricing https://dropsquash.app/refund and the live store.lemonsqueezy.com/checkout/buy/<id> URL".to_string(),
        "preflight final public-release gate later: cargo run -p xtask -- publish-check /absolute/path/to/release-notes.md after production URLs, release notes, and blocker evidence exist".to_string(),
    ]
}
