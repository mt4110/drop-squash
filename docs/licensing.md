# Licensing

Trial usage is based only on successful conversions:

```text
encoder success
AND output exists
AND output size > 0
AND original size > 0
AND output size < original size
```

The app stores a local license cache with:

- license key fingerprint
- local instance name
- Lemon Squeezy instance id
- validation and offline grace timestamps
- validity flag

The app treats the cache as Pro only when it is valid, inside the offline grace
window, and includes both a 64-character hex license fingerprint and Lemon
Squeezy instance id.
When a previously valid cache has identity but the offline grace window has
expired, the app locks conversion with a reconnect prompt instead of presenting
the state as a trial-count failure.
Cache updates are written through a temporary file and then renamed into place.

The trimmed raw license key is sent to Lemon Squeezy only when the user
activates a license in the current desktop flow. It is not persisted in the
config, history, or license cache, and license error messages redact echoed raw
keys before they reach the UI.
The desktop activation form is available during Trial and Locked states, trims
the submitted key, and clears the input before the activation request resolves.
Empty or invalid activation attempts must not create a raw key, license
fingerprint, or Lemon Squeezy instance id in the local cache.

The Lemon Squeezy provider supports validation and deactivation requests when a
raw key is supplied. The desktop app deliberately does not retain that key, so
its current Pro removal action only forgets the local license cache on this Mac.
Server-side deactivation needs a future key-confirmed flow or a customer portal
handoff.

The CLI exposes `dropsquash license status` as a local read-only diagnostic and
`dropsquash license forget` as the matching local cache reset. Status reports
trial usage, Pro/Trial/Locked state, and the license cache path without
contacting Lemon Squeezy or asking for a raw license key. Forget removes the
local cache file only; server-side activation remains unchanged.

Lemon Squeezy sandbox purchase and valid sandbox activation still need manual
verification before paid beta.
