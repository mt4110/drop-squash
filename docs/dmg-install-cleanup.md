# DMG Install Cleanup

## Current Status

DropSquash does not delete or move the downloaded `.dmg` file after a normal Finder install.
This is deliberate: automatic cleanup is not a supported promise for Finder-based installs.

The implemented macOS path is narrower:

- detect when the app is running from a mounted disk image
- show an explicit install action from the disk-image copy
- copy DropSquash to `/Applications` without overwriting an existing app
- keep a post-copy notice visible
- let the user open the installed app
- let the disk-image copy request mounted-volume eject and quit

The downloaded `.dmg` file itself is not moved to Trash yet.

## macOS Constraint

When a user installs with Finder drag-and-drop or copy-and-paste from the DMG to
`/Applications`, DropSquash app code does not receive a reliable copy-complete
event. DropSquash must not promise automatic downloaded-DMG cleanup for the
normal Finder install path.

This is intentional. Guessing from Finder state, Download folder contents, or
file names would be fragile and could move the wrong file.

## Install Path Decision

| Install path | Downloaded-DMG cleanup decision |
|---|---|
| Finder drag-and-drop to `/Applications` | Not supported; DropSquash cannot observe the copy completion or prove the downloaded `.dmg` path. |
| Finder copy-and-paste to `/Applications` | Not supported for the same reason as drag-and-drop. |
| DropSquash in-app install action from the mounted disk image | Allowed later, only as an explicit user action after the installed app and backing `.dmg` are verified. |
| Homebrew cask | Let Homebrew own install and uninstall cleanup. DropSquash must not move Homebrew-managed artifacts. |

## Allowed Product Path

Downloaded-DMG cleanup may only be added to the explicit in-app install flow:

1. The running app is verified to be inside `/Volumes`.
2. The user chooses DropSquash's own install action.
3. The app is copied to `/Applications`.
4. The copied app is verified before any cleanup UI is shown.
5. The installed app is opened through native macOS APIs.
6. The disk-image copy may offer explicit cleanup actions.

Cleanup must be user initiated. The UI should say something like
`Move downloaded DMG to Trash`, not imply automatic deletion. Permanent deletion
is out of scope.

## Implementation Timing

Implement downloaded-DMG Trash movement only after signed and notarized DMG QA
proves all of the following for the same DropSquash distribution image:

- macOS APIs can deterministically return the mounted volume's backing image path
- the backing path is a normal local `.dmg` file
- the file can be verified as the DropSquash distribution image
- missing, remote, renamed, or ambiguous backing paths produce no cleanup action
- the operation moves the `.dmg` to Trash, never permanently deletes it

## Acceptance Criteria

- Finder drag-and-drop install does not claim downloaded-DMG cleanup.
- Finder copy-and-paste install does not claim downloaded-DMG cleanup.
- Cleanup UI appears only after DropSquash's explicit install action succeeds.
- Existing `/Applications/DropSquash.app` is not overwritten.
- Cleanup is disabled when the downloaded `.dmg` path is unknown or unverified.
- Cleanup moves only the verified downloaded `.dmg` to Trash.
- Manual QA records the backing-path evidence and Trash result.
