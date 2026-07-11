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

Record this context with the CSV in `docs/manual-qa.md`:

- DropSquash app version or commit.
- macOS version and machine model.
- Whether the app was run from a packaged `.app` or `xtask`.
- Profile and size arguments.
- Output directory path.

Use this compact format in the manual QA result cell:

```text
Samples: short.mov 0.42x 18.2 MiB/s; medium.mov 0.38x 21.0 MiB/s; large.mov 0.44x 19.5 MiB/s. CSV saved outside repo: /tmp/dropsquash-bench/results.csv
```

Acceptance notes:

- The output must exist and be smaller than the original.
- The app must remain responsive enough for cancellation/manual QA.
- Benchmark outputs and private media samples must not be committed.
- Compare results across the same machine, OS version, profile, and size.

Release candidate rule:

- The first release candidate establishes the baseline for the sample set.
- A later release candidate should not regress throughput by more than 20%
  on two or more samples without a documented reason.
- Any sample that fails to produce a smaller output blocks the release
  candidate until the profile, sample, or encoder behavior is understood.
