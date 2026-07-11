# DropSquash Website

Static sales-site draft for the signed macOS beta.

Open `index.html` in a browser to review the landing page. Public download and checkout links stay intentionally non-live until signing, notarization, and Lemon Squeezy sandbox validation are complete.

Run the static site gate before changing copy or links. It verifies required
sales pages, local links, placeholder URLs, release-status copy, and that
download or checkout URLs are not live before release:

```bash
cargo run -p xtask -- website-check
```
