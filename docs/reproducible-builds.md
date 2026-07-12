# Reproducible Builds

Nix is a developer and CI tool, not a product dependency.

The committed `flake.lock` pins the Apple Silicon macOS/Linux shell. Native Windows builds and tests use a Windows runner because WSL exercises Linux, not Media Foundation or Windows packaging.

The development shell pins Rust 1.95.0, Node 24.16.0, and pnpm 10.34.0 for the
desktop build:

```bash
nix develop --command rustc --version
nix develop --command node -v
nix develop --command pnpm -v
```

Run web checks through the shell when the host Node differs from the pinned
version:

```bash
nix develop --command pnpm --dir apps/desktop/web test
```

Do not commit `.envrc`; release checks intentionally reject local environment
files so secrets and machine-specific shell hooks cannot slip into release
artifacts.

Build the unsigned local QA DMG from the pinned development shell with:

```bash
nix develop --command pnpm --dir apps/desktop tauri build --bundles app,dmg --no-sign --ci
```

Do not add mise or another version manager while Nix can express the required
developer tools. Nix is not a runtime dependency.

Release verification has two stages:

1. Build and compare the unsigned payload from pinned source and dependencies.
2. Sign, notarize, timestamp, and package the verified payload in a protected release environment.

Signing output is not expected to be byte-identical because external timestamp services and signatures add nondeterministic data. Installers must not contain `/nix/store` references, and DropSquash never installs Nix on a user's system:

```bash
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
```

Reference: [Nix reproducible builds](https://reproducible.nixos.org/).
