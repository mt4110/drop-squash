# Future Candidate: Built-In Recording And Privacy Masking

This document preserves a possible future direction without changing the paid
beta scope.

The user direction behind this note is stronger than "add a small feature."
It is a candidate product boundary shift:

```text
DropSquash should eventually be able to record the screen itself,
not only shrink a recording after the fact.
```

## Not In P0

The current paid beta stays focused on one job:

```text
Take an existing Mac screen recording and make it smaller locally.
```

These are not part of the current beta:

- built-in screen recording
- automatic privacy masking
- timeline editing
- enterprise destructive masking and signed audit receipts

## Why This Is Separate

Built-in capture changes DropSquash from a post-processor into a recorder.
Automatic privacy masking raises a separate trust problem: false negatives can
leave private data visible, and false positives can hide needed content.

That means this idea needs its own validation after the paid beta proves the
core post-processing job.

This is not a small extension of the current app flow. It changes:

- permissions and capture UX
- product promise
- failure modes
- support burden
- QA coverage
- release and review expectations

## Candidate Shape

If this direction is explored later, start with a narrow Mac-only shape:

1. ScreenCaptureKit-based recording on macOS.
2. Recorder-first UX: start, stop, save locally, then optional shrink.
3. Optional privacy masking, not mandatory masking.
4. Narrow detection targets first, such as email addresses, URLs, or known app
   regions.
5. Clear user review before export if masking confidence is uncertain.

Do not fork a separate repository at the idea stage. Keep it as a documented
future track in this repository until validation says the recorder boundary is
real and worth carrying long term.

## Entry Criteria

Do not start implementation until all of these are true:

- the paid beta core is shipped and trusted
- Market Validation shows repeat usage for the current post-processing job
- the market decision memo says expansion is justified
- the new recording boundary has its own acceptance criteria and manual QA plan
- the enterprise masking and receipt track in
  `docs/enterprise-audit-masking.md` is accepted if this direction expands into
  audit-ready delivery

## Non-Negotiables

Even in a future phase, keep these rules:

- local-first processing
- no media upload by default
- no `ffmpeg` or `ffprobe` in the product path
- original safety rules stay intact
