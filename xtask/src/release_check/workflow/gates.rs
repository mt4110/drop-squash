pub(super) const CI: &[&str] = &[
    "cargo fmt --all -- --check",
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- release-check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cachix/install-nix-action@v31",
    "nix flake check --no-build --all-systems",
];

pub(super) const RELEASE: &[&str] = &[
    "components: rustfmt, clippy",
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo run -p xtask -- file-size-check",
    "cargo run -p xtask -- website-check",
    "cargo run -p xtask -- manual-qa-check",
    "cargo run -p xtask -- release-check",
    "pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci",
    "cargo run -p xtask -- normalize-dmg target/release/bundle/dmg",
    "cargo run -p xtask -- artifact-check target/release/bundle/dmg/DropSquash.dmg",
    "dropsquash-unsigned-dmg",
    "cargo run -p xtask -- checksum target/release/bundle/dmg/DropSquash.dmg > SHA256SUMS",
    "actions/upload-artifact@v4",
    "dropsquash-unsigned-dmg-checksum",
    "cargo run -p xtask -- macos-signing-check",
    "Block unsigned Phase 0 release",
];

pub(super) const DESKTOP: &[&str] = &[
    "pnpm --dir apps/desktop/web install --frozen-lockfile",
    "pnpm --dir apps/desktop/web lint",
    "pnpm --dir apps/desktop/web build",
    "dtolnay/rust-toolchain@1.95.0",
    "cargo test -p dropsquash-desktop",
];

pub(super) const SECURITY: &[&str] = &[
    "cargo audit",
    "cargo deny check",
    "cargo run -p xtask -- media-policy-check",
    "cargo run -p xtask -- privacy-policy-check",
];
