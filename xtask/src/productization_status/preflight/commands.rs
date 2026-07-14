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
        manual_check(),
        restore_state(),
    ]
}

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
    "preflight after clean: cargo run -p xtask -- manual-qa-prepare --reset-trial --app-artifact target/release/bundle/dmg/DropSquash.dmg --input-sample-set \"short, medium, and large local recordings\" --markdown-output /tmp/dropsquash-manual-qa-prepared.md".to_string()
}

fn checksum() -> String {
    "preflight checksum: cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg --output /tmp/dropsquash-manual-qa-output/SHA256SUMS".to_string()
}

fn benchmark() -> String {
    "preflight benchmark: cargo run -p xtask -- benchmark --release-set --input /absolute/path/to/short.mov --input /absolute/path/to/medium.mov --input /absolute/path/to/large.mov --output-dir /tmp/dropsquash-manual-qa-output --csv-output /tmp/dropsquash-manual-qa-output/benchmark-results.csv".to_string()
}

fn csv_check() -> String {
    "preflight benchmark check: cargo run -p xtask -- benchmark-csv-check /tmp/dropsquash-manual-qa-output/benchmark-results.csv".to_string()
}

fn manual_check() -> String {
    "preflight final gate: cargo run -p xtask -- manual-qa-check".to_string()
}

fn restore_state() -> String {
    "preflight restore state: cargo run -p xtask -- manual-qa-prepare --restore-state".to_string()
}
