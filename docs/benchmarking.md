# Benchmarking

DropSquash benchmarks use the same native encoder path as the desktop app.
They do not use ffmpeg, ffprobe, committed fixtures, telemetry, or cloud upload.

Run benchmarks only against local sample recordings:

```sh
nix develop --command cargo run -p xtask -- benchmark \
  --input ~/Movies/sample-a.mov \
  --input ~/Movies/sample-b.mp4 \
  --output-dir /tmp/dropsquash-bench \
  --profile auto \
  --size auto
```

The command prints CSV to stdout:

```text
input,output,original_bytes,output_bytes,elapsed_s,compression_ratio,throughput_mib_s
```

Use at least three private local samples before a release candidate:

- A short screen recording under 30 seconds.
- A medium recording around 2 to 5 minutes.
- A large recording that is painful to share without compression.

Acceptance notes:

- The output must exist and be smaller than the original.
- The app must remain responsive enough for cancellation/manual QA.
- Benchmark outputs and private media samples must not be committed.
- Compare results across the same machine, OS version, profile, and size.
