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
fn reports_completion_evidence_with_embedded_placeholder() {
    let text = "| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public `DropSquash.dmg` artifact TODO | TBD | Release notes |\n";

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
fn reports_sandbox_purchase_completion_without_intended_product() {
    let text = "| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with test buyer and order | TBD | `docs/manual-qa.md` |\n";

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
fn reports_packaged_manual_qa_without_canonical_artifact_names() {
    let text = "| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `.app` or `.dmg` artifact | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_packaged_manual_qa_without_manual_check() {
    let text = "| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `DropSquash.app` or `DropSquash.dmg` artifact | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Packaged macOS manual QA"));
}

#[test]
fn reports_refund_policy_without_final_policy() {
    let text = "| Refund policy finalized | Blocked | Refund page exists | TBD | `https://...` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Refund policy finalized"));
}

#[test]
fn reports_public_website_completion_without_required_pages() {
    let text = "| Public website deployment | Blocked | Production website serves public pages | TBD | `https://...` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Public website deployment"));
}

#[test]
fn reports_live_checkout_completion_without_product_context() {
    let text = "| Live checkout link | Blocked | Public pricing page opens checkout | TBD | `https://...` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Live checkout link"));
}

#[test]
fn reports_valid_activation_completion_without_cache() {
    let text = "| Valid sandbox activation | Blocked | Activating state disables submit, app reaches Pro state, and raw key is absent | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Valid sandbox activation"));
}

#[test]
fn reports_empty_key_completion_without_cache() {
    let text = "| Empty key activation | Blocked | Activate stays disabled and raw key is absent | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Empty key activation"));
}

#[test]
fn reports_invalid_key_completion_without_friendly_error() {
    let text = "| Invalid license key handling | Blocked | Activating state disables submit and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Invalid license key handling"));
}

#[test]
fn reports_network_failure_completion_without_cache_preservation() {
    let text = "| License network failure | Blocked | Friendly network error appears and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"License network failure"));
}

#[test]
fn reports_local_forget_completion_without_app_state() {
    let text = "| Local license forget | Blocked | Forgetting state disables action and local cache is removed | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Local license forget"));
}

#[test]
fn reports_local_forget_completion_without_cache_removal() {
    let text = "| Local license forget | Blocked | Forgetting state disables action and app returns to trial or locked state | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Local license forget"));
}

#[test]
fn reports_signed_completion_without_developer_id() {
    let text = "| Signed DMG | Blocked | `codesign` verification for the public DropSquash.dmg artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Signed DMG"));
}

#[test]
fn reports_signed_completion_without_public_artifact_name() {
    let text = "| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public DMG artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Signed DMG"));
}

#[test]
fn reports_notarization_completion_without_staple() {
    let text = "| Notarized and stapled DMG | Blocked | `spctl` and notary evidence for the public DropSquash.dmg artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Notarized and stapled DMG"));
}

#[test]
fn reports_notarization_completion_without_public_artifact_name() {
    let text = "| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public DMG artifact | TBD | Release notes |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Notarized and stapled DMG"));
}

#[test]
fn reports_gatekeeper_completion_without_warning_statement() {
    let text = "| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the signed, notarized, stapled app | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn reports_gatekeeper_completion_without_signed_notarized_context() {
    let text = "| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the stapled app without Gatekeeper warning | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Gatekeeper clean-machine open"));
}

#[test]
fn reports_homebrew_completion_without_zap() {
    let text = "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned DropSquash.dmg artifact and cask includes `auto_updates false` | TBD | Homebrew tap PR |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Homebrew cask install"));
}

#[test]
fn reports_checksum_completion_without_public_attachment() {
    let text = "| Published checksum | Blocked | SHA256SUMS with the SHA-256 line for public DropSquash.dmg is generated | TBD | GitHub Release |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Published checksum"));
}

#[test]
fn reports_checksum_completion_without_artifact_name() {
    let text = "| Published checksum | Blocked | SHA256SUMS with the SHA-256 line for public DMG is attached to the release | TBD | GitHub Release |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Published checksum"));
}

#[test]
fn reports_checksum_completion_without_sha256sums() {
    let text = "| Published checksum | Blocked | SHA-256 line for public DropSquash.dmg is attached to the release | TBD | GitHub Release |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Published checksum"));
}

#[test]
fn reports_homebrew_completion_without_versioned_artifact() {
    let text = "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` works and cask includes `auto_updates false` plus `zap` cleanup | TBD | Homebrew tap PR |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Homebrew cask install"));
}

#[test]
fn reports_homebrew_completion_without_dmg_name() {
    let text = "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact and cask includes `auto_updates false` plus `zap` cleanup | TBD | Homebrew tap PR |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Homebrew cask install"));
}

#[test]
fn reports_homebrew_completion_without_auto_update_policy() {
    let text = "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned DropSquash.dmg artifact and cask includes `zap` cleanup | TBD | Homebrew tap PR |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Homebrew cask install"));
}

#[test]
fn reports_benchmark_completion_without_threshold() {
    let text = "| Benchmark release set | Blocked | Release-set benchmark CSV covers short, medium, and large local samples, smaller outputs, and machine/OS context | TBD | `docs/manual-qa.md` |\n";

    let incomplete = incomplete_requirements(text);

    assert!(incomplete.contains(&"Benchmark release set"));
}

fn described_blockers() -> String {
    [
        "| Packaged macOS manual QA | Blocked | Filled manual QA table for the exact `DropSquash.app` or `DropSquash.dmg` artifact, with `manual-qa-check` passing | TBD | `docs/manual-qa.md` |\n",
        "| Lemon Squeezy product setup | Blocked | Sandbox product is configured for DropSquash with license keys enabled | TBD | `docs/manual-qa.md` |\n",
        "| Lemon Squeezy sandbox purchase | Blocked | Sandbox checkout completes with the intended product, test buyer, and order | TBD | `docs/manual-qa.md` |\n",
        "| Empty key activation | Blocked | Activate stays disabled for empty input and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n",
        "| Valid sandbox activation | Blocked | Activating state disables submit, app reaches Pro state, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n",
        "| Invalid license key handling | Blocked | Activating state disables submit, friendly error appears, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n",
        "| License network failure | Blocked | Friendly network error appears, existing valid local cache remains intact, and raw key is absent from local cache | TBD | `docs/manual-qa.md` |\n",
        "| Local license forget | Blocked | Forgetting state disables action, local cache is removed, and app returns to trial or locked state | TBD | `docs/manual-qa.md` |\n",
        "| Public website deployment | Blocked | Production website serves the release-status, privacy, pricing, support, and download pages | TBD | `https://...` |\n",
        "| Refund policy finalized | Blocked | Production refund policy is final and linked before checkout goes live | TBD | `https://...` |\n",
        "| Live checkout link | Blocked | Public pricing page opens the tested Lemon Squeezy checkout for the intended product | TBD | `https://...` |\n",
        "| Signed DMG | Blocked | `codesign` verification shows Developer ID for the public `DropSquash.dmg` artifact | TBD | Release notes |\n",
        "| Notarized and stapled DMG | Blocked | `spctl`, notary, and stapled evidence for the public `DropSquash.dmg` artifact | TBD | Release notes |\n",
        "| Gatekeeper clean-machine open | Blocked | Fresh macOS account or clean machine opens the signed, notarized, stapled app without Gatekeeper warning | TBD | `docs/manual-qa.md` |\n",
        "| Benchmark release set | Blocked | Release-set benchmark CSV covers short, medium, and large local samples, smaller outputs, machine/OS context, and 20% regression threshold | TBD | `docs/manual-qa.md` |\n",
        "| Published checksum | Blocked | SHA256SUMS with the SHA-256 line for public `DropSquash.dmg` is attached to the release | TBD | GitHub Release |\n",
        "| Homebrew cask install | Blocked | `brew install --cask mt4110/tap/dropsquash` installs the versioned artifact `DropSquash.dmg` and cask includes `auto_updates false` plus `zap` cleanup | TBD | Homebrew tap PR |\n",
    ]
    .join("")
}
