# Build Week Devpost Checklist

Use this checklist while filling Devpost. Do not treat checked boxes as proof of
final submission; the Devpost confirmation page is the final proof.

## Before Opening Devpost

- [ ] Run `cargo run -p xtask -- build-week-local-submit-check`.
- [ ] Confirm the demo video is public or unlisted-public and playable.
- [ ] Copy the demo video URL.
- [ ] Run `/feedback` in the main Codex build thread.
- [ ] Copy the `/feedback` session ID.
- [ ] Confirm `devposttesting` has repository read access.
- [ ] Confirm `build-week-event@openai.com` has a pending or accepted read
      invitation.

## Devpost Fields

- [ ] Project name:
      `DropSquash Secure Share Alpha`
- [ ] Track:
      `Developer Tools`
- [ ] Elevator pitch:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] Built with:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] Repository URL:
      `https://github.com/mt4110/drop-squash`
- [ ] Demo URL:
      paste the public demo video URL.
- [ ] Short description:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] What it does:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] Why it matters:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] Demo evidence:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] How Codex and GPT-5.6 were used:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] What is not finished:
      copy from `docs/build-week-devpost-fields.md`.
- [ ] `/feedback` session ID:
      paste the copied session ID.

## Claim Boundary

- [ ] The submission does not claim complete privacy protection.
- [ ] The submission does not claim enterprise audit readiness.
- [ ] The submission does not claim completed selective masking.
- [ ] The submission says recognized text and observation geometry are not
      stored.
- [ ] The submission says video URL, `/feedback`, and Devpost submit are manual
      external steps until actually completed.

## After Submit

- [ ] Save or screenshot the Devpost confirmation page.
- [ ] Record the final Devpost project URL.
- [ ] Record the final demo video URL.
- [ ] Record the final `/feedback` session ID.
- [ ] Rerun `cargo run -p xtask -- build-week-local-submit-check` if any local
      artifact changed before final submit.
