import assert from "node:assert/strict";
import { access, readFile } from "node:fs/promises";
import test from "node:test";

async function render(path = "/") {
  const workerUrl = new URL("../dist/server/index.js", import.meta.url);
  workerUrl.searchParams.set("test", `${process.pid}-${Date.now()}-${path}`);
  const { default: worker } = await import(workerUrl.href);
  return worker.fetch(
    new Request(`http://localhost${path}`, {
      headers: { accept: "text/html" },
    }),
    {
      ASSETS: {
        fetch: async () => new Response("Not found", { status: 404 }),
      },
    },
    { waitUntil() {}, passThroughOnException() {} },
  );
}

test("server-renders the DropSquash home page", async () => {
  const response = await render("/");
  assert.equal(response.status, 200);
  assert.match(response.headers.get("content-type") ?? "", /^text\/html\b/i);
  const html = await response.text();
  assert.match(html, /<title>DropSquash<\/title>/i);
  assert.match(
    html,
    /DropSquash is the automatic local post-processor for Mac screen recordings\./,
  );
  assert.match(html, /20 successful smaller conversions free/i);
  assert.doesNotMatch(html, /codex-preview|SkeletonPreview|react-loading-skeleton/i);
});

test("server-renders the pricing page", async () => {
  const response = await render("/pricing");
  assert.equal(response.status, 200);
  const html = await response.text();
  assert.match(html, /<title>Pricing - DropSquash<\/title>/i);
  assert.match(html, /\$29/);
  assert.match(
    html,
    /The live checkout URL will be published here after commerce validation completes\./,
  );
  assert.match(
    html,
    /Failed, cancelled, or larger-result conversions do not count toward the trial\./,
  );
});

test("server-renders the release-status page with the paid-beta scope", async () => {
  const response = await render("/release-status");
  assert.equal(response.status, 200);
  const html = await response.text();
  assert.match(html, /<title>Release Status - DropSquash<\/title>/i);
  assert.match(
    html,
    /public release には published checksum、manual QA evidence、public web finalization、live checkout validation が必要です/u,
  );
  assert.match(
    html,
    /Built-in screen recording と automatic privacy masking は future candidates であり、current release promise ではありません/u,
  );
  assert.match(html, /docs\/release-blockers\.md/);
  assert.match(html, /href="\/pricing"/);
  assert.match(html, /href="\/refund"/);
  assert.match(html, /href="\/download"/);
});

test("server-renders the support and download pages with current scope limits", async () => {
  const support = await render("/support");
  const download = await render("/download");
  assert.equal(support.status, 200);
  assert.equal(download.status, 200);

  const supportHtml = await support.text();
  const downloadHtml = await download.text();

  assert.match(supportHtml, /<title>Support - DropSquash<\/title>/i);
  assert.match(
    supportHtml,
    /現在の公開対象は macOS のみです。MOV \/ MP4 \/ M4V の画面収録を、single-file の local post-processing で smaller MP4 にします/u,
  );
  assert.match(
    supportHtml,
    /The refund policy is published before checkout goes live so the purchase path can be reviewed in advance\./,
  );
  assert.match(downloadHtml, /<title>Download - DropSquash<\/title>/i);
  assert.match(
    downloadHtml,
    /Signed、notarized、clean-machine verified artifact は揃っていますが、public download は checksum publication と release publication が整うまで待機します/u,
  );
  assert.match(
    downloadHtml,
    /Windows と Linux の実装作業は repository にありますが、public conversion support はまだ claim しません/u,
  );
});

test("server-renders privacy, terms, and license pages with local-first limits", async () => {
  const privacy = await render("/privacy");
  const terms = await render("/terms");
  const license = await render("/license");
  assert.equal(privacy.status, 200);
  assert.equal(terms.status, 200);
  assert.equal(license.status, 200);

  const privacyHtml = await privacy.text();
  const termsHtml = await terms.text();
  const licenseHtml = await license.text();

  assert.match(privacyHtml, /<title>Privacy - DropSquash<\/title>/i);
  assert.match(privacyHtml, /DropSquash does not upload media for compression\./);
  assert.match(privacyHtml, /Telemetry is off by default\./);
  assert.match(termsHtml, /<title>Terms of Service - DropSquash<\/title>/i);
  assert.match(
    termsHtml,
    /Public download と checkout は checksum publication、release evidence、production URL readiness、commerce validation が揃うまで無効です/u,
  );
  assert.match(licenseHtml, /<title>License - DropSquash<\/title>/i);
  assert.match(licenseHtml, /raw license key 自体は保持しません/u);
  assert.match(
    licenseHtml,
    /successful validation 後は offline grace を許可します。grace が切れた場合は、一度再接続してその Mac の Pro 状態を更新してください/u,
  );
});

test("server-renders the refund page with manual beta policy details", async () => {
  const refund = await render("/refund");
  assert.equal(refund.status, 200);
  const html = await refund.text();
  assert.match(html, /<title>Refund Policy - DropSquash<\/title>/i);
  assert.match(
    html,
    /private \/ manual paid beta 中の refund request は support review 後に手動対応します。Manual handling remains the current path/u,
  );
  assert.match(
    html,
    /public checkout 開始後の refund request は Lemon Squeezy order flow を通じて確認します。Until then, no public checkout URL is published/u,
  );
  assert.match(html, /href="\/support"/);
});

test("starter preview files are removed", async () => {
  const layout = await readFile(new URL("../app/layout.tsx", import.meta.url), "utf8");
  assert.match(layout, /title:\s*"DropSquash"/);
  assert.doesNotMatch(layout, /Starter Project|codex-preview|_sites-preview/);
  await assert.rejects(
    access(new URL("../app/_sites-preview/SkeletonPreview.tsx", import.meta.url)),
  );
  await assert.rejects(
    access(new URL("../app/_sites-preview/preview.css", import.meta.url)),
  );
});
