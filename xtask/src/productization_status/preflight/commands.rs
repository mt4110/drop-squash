mod distribution;
mod public_web;

pub(super) fn local_packaged_app() -> Vec<String> {
    vec![
        install_desktop(),
        install_web(),
        build(),
        normalize(),
        artifact_check(),
        manual_qa(),
        "preflight markdown-output note: keep the prepared markdown on a new outside-repo path such as /tmp/dropsquash-manual-qa-prepared-<app-build>.md so one QA run cannot overwrite another".to_string(),
        checksum(),
        benchmark(),
        csv_check(),
        ready_all(),
        ready_local_proof(),
        pending_local_proof(),
        manual_check(),
        restore_state(),
    ]
}

pub(super) fn license_sandbox() -> Vec<String> {
    vec![
        format!(
            "preflight prepare markdown: cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --app-state-dir \"{APP_STATE_DIR}\" --state-dir {STATE_DIR} --output-dir {OUTPUT_DIR} --markdown-output {QA_MARKDOWN}"
        ),
        ready_all(),
        "preflight combined rerun helper: cargo run -p xtask -- manual-qa-paid-beta-rerun".to_string(),
        format!(
            "preflight ready license: cargo run -p xtask -- manual-qa-license-rerun {QA_MARKDOWN}"
        ),
        format!("preflight license markdown rows: cargo run -p xtask -- manual-qa-pending {QA_MARKDOWN} --section license | rg '^license (product setup|sandbox purchase|valid activation) markdown row:'"),
        "preflight paid beta gate: cargo run -p xtask -- paid-beta-check".to_string(),
        "preflight license diagnostics: cargo run -p dropsquash -- license status".to_string(),
        "preflight license manual gate: cargo run -p xtask -- manual-qa-check".to_string(),
    ]
}

pub(super) fn public_web() -> Vec<String> {
    public_web::lines()
}

pub(super) fn distribution_signing() -> Vec<String> {
    distribution::lines()
}

pub(super) const APP_STATE_DIR: &str =
    "/tmp/dropsquash-manual-qa-app-state/Library/Application Support/DropSquash";
pub(super) const STATE_DIR: &str = "/tmp/dropsquash-manual-qa-state";
pub(super) const OUTPUT_DIR: &str = "/tmp/dropsquash-manual-qa-output";
pub(super) const QA_MARKDOWN: &str = "/tmp/dropsquash-manual-qa-prepared-<app-build>.md";
const RESULTS_HINT: &str = "/tmp/dropsquash-manual-qa-output/benchmark-results-<app-build>.csv";

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
        "preflight benchmark: cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir {OUTPUT_DIR} --csv-output {RESULTS_HINT}"
    )
}

fn csv_check() -> String {
    format!("preflight benchmark check: cargo run -p xtask -- benchmark-csv-check {RESULTS_HINT}")
}

pub(super) fn ready_all() -> String {
    format!(
        "preflight deterministic helper: cargo run -p xtask -- manual-qa-ready-all {QA_MARKDOWN} {RESULTS_HINT}"
    )
}

fn ready_local_proof() -> String {
    format!(
        "preflight packaged-only alternative: cargo run -p xtask -- manual-qa-ready-local-proof {QA_MARKDOWN} {RESULTS_HINT} (or, if docs/manual-qa.md already records the checked CSV, run cargo run -p xtask -- manual-qa-packaged-rerun; then use the emitted sample-link and installed-app helper commands before chooser or mounted-DMG checks)"
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
