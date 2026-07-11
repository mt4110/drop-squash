use super::incomplete_requirements;

#[test]
fn accepts_described_completion_evidence() {
    let text = described_blockers();

    assert!(incomplete_requirements(&text).is_empty());
}

#[test]
fn reports_tbd_completion_evidence() {
    let text = "| Signed DMG | Blocked | TBD | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Signed DMG"));
}

#[test]
fn reports_vague_completion_evidence() {
    let text = "| Signed DMG | Blocked | Evidence required | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Signed DMG"));
}

#[test]
fn reports_sandbox_purchase_completion_without_order() {
    let text = "| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product and test buyer | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Lemon Squeezy sandbox purchase"));
}

#[test]
fn reports_product_setup_without_license_keys() {
    let text = "| Lemon Squeezy product setup | Blocked | Sandbox product is configured for DropSquash | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Lemon Squeezy product setup"));
}

#[test]
fn reports_refund_policy_without_final_policy() {
    let text = "| Refund policy finalized | Blocked | Refund page exists | TBD | `https://...` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Refund policy finalized"));
}

#[test]
fn reports_valid_activation_completion_without_cache() {
    let text = "| Valid sandbox activation | Blocked | App reaches Pro state and raw key is absent | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_invalid_key_completion_without_friendly_error() {
    let text = "| Invalid license key handling | Blocked | raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Invalid license key handling"));
}

#[test]
fn reports_local_forget_completion_without_app_state() {
    let text =
        "| Local license forget | Blocked | Local cache is removed | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Local license forget"));
}

#[test]
fn reports_signed_completion_without_developer_id() {
    let text = "| Signed DMG | Blocked | `codesign` verification for the public DMG artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Signed DMG"));
}

#[test]
fn reports_notarization_completion_without_staple() {
    let text = "| Notarized and stapled DMG | Blocked | `spctl` and notary evidence for the public DMG artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Notarized and stapled DMG"));
}

#[test]
fn reports_homebrew_completion_without_zap() {
    let text = "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact | TBD | Homebrew tap PR |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Homebrew cask install"));
}

fn described_blockers() -> String {
    [
        "| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `.app` or `.dmg` artifact | TBD | `docs/manual-qa.md` |\n",
        "| Lemon Squeezy product setup | Blocked | Sandbox product is configured for DropSquash with license keys enabled | TBD | `docs/manual-qa.md` |\n",
        "| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product, test buyer, and order | TBD | `docs/manual-qa.md` |\n",
        "| Valid sandbox activation | Blocked | App reaches Pro state and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n",
        "| Public website deployment | Blocked | Production website serves the release-status, privacy, pricing, support, and download pages | TBD | `https://...` |\n",
        "| Refund policy finalized | Blocked | Production refund policy is final and linked before checkout goes live | TBD | `https://...` |\n",
        "| Live checkout link | Blocked | Public pricing page opens the tested Lemon Squeezy checkout for the intended product | TBD | `https://...` |\n",
        "| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public DMG artifact | TBD | Release notes |\n",
        "| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public DMG artifact | TBD | Release notes |\n",
        "| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the stapled app | TBD | `docs/manual-qa.md` |\n",
        "| Published checksum | Blocked | SHA-256 line for the public DMG is attached to the release | TBD | GitHub Release |\n",
        "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact and cask includes `zap` cleanup | TBD | Homebrew tap PR |\n",
    ]
    .join("")
}
