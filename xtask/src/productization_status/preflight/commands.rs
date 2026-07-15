pub(super) fn local_packaged_app() -> Vec<String> {
    vec![
        install_desktop(),
        install_web(),
        build(),
        normalize(),
        artifact_check(),
        manual_qa(),
        checksum(),
        benchmark(),
        csv_check(),
        ready_local_proof(),
        pending_local_proof(),
        manual_check(),
        restore_state(),
    ]
}

pub(super) fn license_sandbox() -> Vec<String> {
    vec![
        format!("preflight license pending: cargo run -p xtask -- manual-qa-pending {QA_MARKDOWN} --section license"),
        "preflight license diagnostics: cargo run -p dropsquash -- license status".to_string(),
        "preflight license manual gate: cargo run -p xtask -- manual-qa-check".to_string(),
    ]
}

pub(super) fn public_web() -> Vec<String> {
    vec![
        "preflight public web gate: cargo run -p xtask -- website-check".to_string(),
        "preflight publish gate: cargo run -p xtask -- publish-check /absolute/path/to/release-notes.md".to_string(),
        "preflight production URLs: verify https://dropsquash.app/release-status https://dropsquash.app/pricing https://dropsquash.app/refund and the live store.lemonsqueezy.com/checkout/buy/<id> URL".to_string(),
    ]
}

pub(super) fn distribution_signing() -> Vec<String> {
    vec![
        format!("preflight distribution pending: cargo run -p xtask -- manual-qa-pending {QA_MARKDOWN} --section distribution"),
        "preflight signing environment: cargo run -p xtask -- macos-signing-check".to_string(),
        "preflight distribution manual gate: cargo run -p xtask -- manual-qa-check".to_string(),
    ]
}

const APP_STATE_DIR: &str =
    "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash";
const STATE_DIR: &str = "/tmp/dropsquash-manual-qa-state";
const OUTPUT_DIR: &str = "/tmp/dropsquash-manual-qa-output";
const QA_MARKDOWN: &str = "/tmp/dropsquash-manual-qa-prepared.md";

fn install_desktop() -> String {
    "preflight install desktop deps: nix develop --command pnpm --dir apps/desktop install --frozen-lockfile".to_string()
}

fn install_web() -> String {
    "preflight install web deps: nix develop --command pnpm --dir apps/desktop/web install --frozen-lockfile".to_string()
}

fn build() -> String {
    "preflight build: nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci".to_string()
}

fn normalize() -> String {
    "preflight normalize: cargo run -p xtask -- normalize-dmg target/release/bundle/dmg".to_string()
}

fn artifact_check() -> String {
    "preflight artifact check: cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg".to_string()
}

fn manual_qa() -> String {
    format!(
        "preflight after clean: cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"{APP_STATE_DIR}\" --state-dir {STATE_DIR} --output-dir {OUTPUT_DIR} --markdown-output {QA_MARKDOWN}"
    )
}

fn checksum() -> String {
    format!(
        "preflight checksum: cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output {OUTPUT_DIR}/SHA256SUMS"
    )
}

fn benchmark() -> String {
    format!(
        "preflight benchmark: cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir {OUTPUT_DIR} --csv-output {OUTPUT_DIR}/benchmark-results.csv"
    )
}

fn csv_check() -> String {
    format!(
        "preflight benchmark check: cargo run -p xtask -- benchmark-csv-check {OUTPUT_DIR}/benchmark-results.csv"
    )
}

fn ready_local_proof() -> String {
    format!(
        "preflight ready local proof: cargo run -p xtask -- manual-qa-ready-local-proof {QA_MARKDOWN} {OUTPUT_DIR}/benchmark-results.csv"
    )
}

fn pending_local_proof() -> String {
    format!(
        "preflight local-proof pending: cargo run -p xtask -- manual-qa-pending {QA_MARKDOWN} --section local-proof"
    )
}

fn manual_check() -> String {
    "preflight final gate: cargo run -p xtask -- manual-qa-check".to_string()
}

fn restore_state() -> String {
    format!(
        "preflight restore state: cargo run -p xtask -- manual-qa-prepare --restore-state --app-state-dir \"{APP_STATE_DIR}\" --state-dir {STATE_DIR}"
    )
}
