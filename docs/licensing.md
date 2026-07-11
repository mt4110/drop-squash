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

The raw license key is sent only to Lemon Squeezy License API activation,
validation, and deactivation endpoints. It is not persisted in the config,
history, or license cache.

Lemon Squeezy sandbox activation still needs manual verification before paid beta.
