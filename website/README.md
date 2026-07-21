# DropSquash Website

Static sales-site draft for the signed macOS beta.

Open `index.html` in a browser to review the landing page. Public download and checkout links stay intentionally non-live until signing, notarization, Lemon Squeezy sandbox validation, and final price confirmation are complete.
The public website deployment and live checkout link stay blocked in
`docs/release-blockers.md` until production URLs are verified.
This repository currently ships only the static site files under `website/`.
It now includes `.openai/hosting.json` for the Sites project, but a local
preview is still not deployment proof for `https://dropsquash.app/...`.
The deployable production-host surface now lives in `apps/site`.
Use
[docs/website-deployment-runbook.md](/Users/masakitakemura/_workspace/drop-squash/docs/website-deployment-runbook.md)
for the production-host verification sequence.

Run the static site gate before changing copy or links. It verifies required
sales pages, local links and resources, local link fragments, CSS `url(...)` resources,
`srcset` candidates, video posters, quoted and unquoted HTML links, form actions,
approved external-link host/path boundaries, placeholder URLs, unsupported platform availability claims
including short download CTAs and natural release copy, release-status page,
privacy, terms, license, refund, support contact copy, and
pre-release CTA copy. It also keeps download or checkout links and form actions non-live
before release, and keeps artifact resources non-live:

```bash
cargo run -p xtask -- website-check
```
