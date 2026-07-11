# Contributing

This project is in early private development.

Before opening a change, run:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
nix flake check --no-build --all-systems
```

Keep business logic out of UI code. Original-file handling must remain conservative and test-covered.

Do not add an external media executable, shell invocation, or runtime plugin download. Native backend changes require tests on the corresponding operating system. Use Nix for pinned development and CI versions when possible; do not add a second version manager unless Nix cannot express a required tool. Nix is not a runtime dependency.
