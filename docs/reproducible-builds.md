# Reproducible Builds

Nix is a developer and CI tool, not a product dependency.

The committed `flake.lock` pins the Apple Silicon macOS/Linux shell. Native Windows builds and tests use a Windows runner because WSL exercises Linux, not Media Foundation or Windows packaging.

The development shell also pins Node 24 and pnpm 10 for the desktop web build:

```bash
nix develop --command rustc --version
nix develop --command node -v
nix develop --command pnpm -v
```

Release verification has two stages:

1. Build and compare the unsigned payload from pinned source and dependencies.
2. Sign, notarize, timestamp, and package the verified payload in a protected release environment.

Signing output is not expected to be byte-identical because external timestamp services and signatures add nondeterministic data. Installers must not contain `/nix/store` references, and DropSquash never installs Nix on a user's system:

```bash
cargo run -p xtask -- artifact-check path/to/DropSquash.dmg
```

Reference: [Nix reproducible builds](https://reproducible.nixos.org/).
