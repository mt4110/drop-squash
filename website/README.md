# DropSquash Website

Static sales-site draft for the signed macOS beta.

Open `index.html` in a browser to review the landing page. Public download and checkout links stay intentionally non-live until signing, notarization, Lemon Squeezy sandbox validation, and final price confirmation are complete.
The public website deployment and live checkout link stay blocked in
`docs/release-blockers.md` until production URLs are verified.

Run the static site gate before changing copy or links. It verifies required
sales pages, local links and resources, quoted and unquoted HTML links,
form actions, approved external-link host/path boundaries, placeholder URLs,
unsupported platform availability claims including short download CTAs and natural release copy, release-status page, privacy,
license, refund, support contact copy, and pre-release CTA copy. It also keeps
download or checkout links and form actions non-live before release:

```bash
cargo run -p xtask -- website-check
```
