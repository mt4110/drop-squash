# DropSquash Site

This is the Sites-compatible public web surface for DropSquash.

It exists so the product's public pages can move from the static `website/`
draft into a deployable build that emits:

- `dist/server/index.js`
- `dist/.openai/hosting.json`

Use this app for the production `dropsquash.app` surface after:

1. `cargo run -p xtask -- public-web-rerun`
2. `cargo run -p xtask -- website-check`
3. `npm install`
4. `npm run verify:site`
5. `cargo run -p xtask -- publish-check path/to/release-notes.md` before the final public release decision

The public product truth still comes from the shipped app, release blockers, and
manual QA evidence. Do not treat a local preview as production deployment proof.
