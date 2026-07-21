pub(super) fn lines() -> [&'static str; 14] {
    [
        "distribution final check markdown row: | `cargo run -p xtask -- manual-qa-check` | Passes after every manual QA result is recorded | manual-qa-check passed |",
        "distribution final check row candidate: replace the manual-qa-check row only after every manual QA result is filled and the checker passes",
        "distribution homebrew markdown row: | `cargo run -p xtask -- homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb path/to/release-notes.md` | Generated `dropsquash.rb` cask matches `.md` release notes version, Artifact URL, SHA-256, `auto_updates false`, and `zap` | homebrew-cask-check passed for packaging/homebrew/Casks/dropsquash.rb and /absolute/path/to/release-notes.md |",
        "distribution homebrew row candidate: replace the Homebrew row only after a public release notes markdown and exact Artifact URL exist",
        "distribution signing environment markdown row: | `cargo run -p xtask -- macos-signing-check` | Passes in release environment | macos-signing-check passed in release environment |",
        "distribution signing environment row candidate: replace the signing-environment row only after release secrets are loaded and macos-signing-check passes",
        "distribution codesign row candidate: codesign verified Developer ID Application signature for public DropSquash.dmg",
        "distribution codesign markdown row: | Codesign verification | Public DMG/app artifact verifies with Developer ID signature | codesign verified Developer ID Application signature for public DropSquash.dmg |",
        "distribution notarization row candidate: notary accepted, stapler validate passed, and spctl accepted for public DropSquash.dmg",
        "distribution notarization markdown row: | Notarization staple verification | Public DMG/app artifact passes notary, stapler validate or stapled status, and `spctl` assessment | notary accepted, stapler validate passed, and spctl accepted for public DropSquash.dmg |",
        "distribution Gatekeeper row candidate: Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh macOS account without Gatekeeper warning",
        "distribution Gatekeeper markdown row: | Gatekeeper open test | Signed, notarized, stapled app from public `DropSquash.dmg` matching the release notes Artifact URL opens cleanly without Gatekeeper warning | Gatekeeper opened signed, notarized, stapled app from public DropSquash.dmg cleanly in fresh macOS account without Gatekeeper warning |",
        "distribution release-url note: replace /absolute/path/to/release-notes.md in the Homebrew row after the public release notes markdown exists",
        "distribution public-artifact note: replace public DropSquash.dmg wording only after the signed artifact matches the release notes Artifact URL",
    ]
}
