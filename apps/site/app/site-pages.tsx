import Link from "next/link";
import type { Metadata } from "next";
import { SiteShell, type NavLink } from "./site-shell";

type PageSpec = {
  titleJa: string;
  titleEn: string;
  metadataTitle: string;
  description: string;
  navLinks: NavLink[];
  pageClassName?: string;
  footer?: string;
  render: () => JSX.Element;
};

const mainNav = nav(
  link("ダウンロード", "Download", "/download"),
  link("価格", "Pricing", "/pricing"),
  link("利用規約", "Terms", "/terms"),
  link("ライセンス", "License", "/license"),
  link("プライバシー", "Privacy", "/privacy"),
  link("サポート", "Support", "/support"),
);
const supportNav = nav(
  link("ダウンロード", "Download", "/download"),
  link("価格", "Pricing", "/pricing"),
  link("利用規約", "Terms", "/terms"),
  link("プライバシー", "Privacy", "/privacy"),
  link("サポート", "Support", "/support"),
);
const legalNav = nav(
  link("ダウンロード", "Download", "/download"),
  link("価格", "Pricing", "/pricing"),
  link("利用規約", "Terms", "/terms"),
  link("ライセンス", "License", "/license"),
  link("サポート", "Support", "/support"),
);

const pages: Record<string, PageSpec> = {
  "": {
    titleJa: "DropSquash",
    titleEn: "DropSquash",
    metadataTitle: "DropSquash",
    description:
      "Mac画面収録をローカルで軽くする / Automatic local post-processing for Mac screen recordings.",
    navLinks: mainNav,
    footer:
      "© DropSquash. 動画はローカルのままです / Video stays local; license activation contacts Lemon Squeezy only when you use it.",
    render: renderHome,
  },
  pricing: page("Pricing - DropSquash", "価格", "Pricing", legalNav, renderPricing),
  privacy: page("Privacy - DropSquash", "プライバシー", "Privacy", supportNav, renderPrivacy),
  terms: page("Terms of Service - DropSquash", "利用規約", "Terms of Service", nav(
    link("ダウンロード", "Download", "/download"),
    link("価格", "Pricing", "/pricing"),
    link("プライバシー", "Privacy", "/privacy"),
    link("サポート", "Support", "/support"),
  ), renderTerms),
  support: page("Support - DropSquash", "サポート", "Support", nav(
    link("ダウンロード", "Download", "/download"),
    link("価格", "Pricing", "/pricing"),
    link("利用規約", "Terms", "/terms"),
    link("プライバシー", "Privacy", "/privacy"),
    link("ライセンス", "License", "/license"),
  ), renderSupport),
  license: page("License - DropSquash", "ライセンス方針", "License policy", supportNav, renderLicense),
  refund: page("Refund Policy - DropSquash", "返金方針", "Refund policy", legalNav, renderRefund),
  download: page("Download - DropSquash", "ダウンロード", "Download", nav(
    link("価格", "Pricing", "/pricing"),
    link("利用規約", "Terms", "/terms"),
    link("ライセンス", "License", "/license"),
    link("プライバシー", "Privacy", "/privacy"),
    link("サポート", "Support", "/support"),
  ), renderDownload),
  "release-status": page("Release Status - DropSquash", "公開状況", "Release status", nav(
    link("ダウンロード", "Download", "/download"),
    link("価格", "Pricing", "/pricing"),
    link("利用規約", "Terms", "/terms"),
    link("ライセンス", "License", "/license"),
    link("サポート", "Support", "/support"),
  ), renderReleaseStatus),
};

export function resolvePage(path: string) {
  return pages[path];
}

export function buildMetadata(path: string): Metadata {
  const page = pages[path];
  return {
    title: page.metadataTitle,
    description: page.description,
  };
}

export function renderPage(path: string) {
  const page = pages[path];
  return (
    <SiteShell
      footer={page.footer}
      navLinks={page.navLinks}
      pageClassName={page.pageClassName}
    >
      {page.render()}
    </SiteShell>
  );
}

function page(
  metadataTitle: string,
  headingJa: string,
  headingEn: string,
  navLinks: NavLink[],
  renderBody: () => JSX.Element,
): PageSpec {
  return {
    titleJa: headingJa,
    titleEn: headingEn,
    metadataTitle,
    description: `${headingJa} / ${headingEn} for the DropSquash macOS paid beta.`,
    navLinks,
    pageClassName: "page",
    render: () => (
      <>
        <h1>
          <span className="copy-ja">{headingJa}</span>
          <span className="copy-en">{headingEn}</span>
        </h1>
        {renderBody()}
      </>
    ),
  };
}

function renderHome() {
  return (
    <>
      <section className="hero">
        <div>
          <p className="eyebrow">macOS有料ベータ準備中 / macOS paid beta in preparation</p>
          <h1>
            <span className="copy-ja">Mac画面収録を、ローカルのまま軽くする。</span>
            <span className="copy-en">DropSquash is the automatic local post-processor for Mac screen recordings.</span>
          </h1>
          <p className="lead">
            <span className="copy-ja">録画を1本入れると、より小さいMP4を1本返します。元動画は安全に保持し、変換のために動画をアップロードしません。</span>
            <span className="copy-en">Drop one recording, get one smaller MP4, keep the original safe, and never upload media for conversion.</span>
          </p>
          <div className="actions">
            <Link className="button" href="/download">
              ダウンロード状況 / Download status
            </Link>
            <Link className="button secondary" href="/pricing">
              価格 / Pricing
            </Link>
          </div>
          <ul className="promise-list">
            <li>MOV / MP4 / M4V 入力対応 on macOS</li>
            <li>AVFoundationベースのローカル変換 / AVFoundation-based local conversion</li>
            <li>有効な縮小変換 20 回まで無料 / 20 successful smaller conversions free</li>
          </ul>
        </div>
        <div aria-label="DropSquash app preview" className="app-shot">
          <div className="drop">
            <p>
              <strong>録画をドロップ / Drop Recording</strong>
              <span>MOV / MP4 / M4V here</span>
              <span className="bar">
                <i />
              </span>
            </p>
          </div>
        </div>
      </section>
      <section className="band">
        <CopyBlock
          title="何をするものか / What it does"
          text="DropSquash は、チャットやドキュメントや issue tracker に共有する前に、Mac の画面収録を軽くするための小さなデスクトップアプリです。The current product scope is intentionally narrow: one recording in, one smaller MP4 out."
        />
        <CopyBlock
          title="今はやらないこと / What it does not do"
          text="変換のために動画をアップロードせず、製品経路で command-line media tools を実行せず、現在のベータでは built-in screen recording や automatic privacy masking を含みません。It also does not claim unsupported Windows or Linux conversion yet or delete originals by default."
        />
      </section>
      <section className="grid">
        <Card
          title="ローカル優先 / Local first"
          text="変換はあなたの Mac 上で完結します。Media is not uploaded for conversion."
        />
        <Card
          title="元ファイル保護 / Original safe"
          text="検証済み変換のあとに明示的にゴミ箱へ移動しない限り、元ファイルは残ります。Originals stay put by default."
        />
        <Card
          title="共有向け / Made for sharing"
          text="プロフィールは chat / docs / archive の定番用途に絞っています。No cloud dependency is added."
        />
      </section>
      <section className="band">
        <CopyBlock
          title="公開前の前提 / Before public release"
          text="DropSquash は厳密な順番で商品化しています。公開情報、法務ページ、価格期待、配布準備が実アプリと一致するまで checkout は開きません。The paid beta still waits on checksum publication, release blocker verification, and commerce validation before public checkout opens."
          linkHref="/release-status"
          linkLabel="公開状況を見る / Review Release status"
        />
        <CopyBlock
          title="サポート / Support"
          text="現時点のサポート窓口は GitHub Issues です。Privacy policy, terms, license policy, refund policy, pricing, release status, and download status are public first so the product surface can be inspected before payment is enabled."
          linkHref="/support"
          linkLabel="サポート情報 / Open support notes"
        />
      </section>
    </>
  );
}

function renderPricing() {
  return (
    <div className="card">
      <h2>Pro ライセンス / Pro license</h2>
      <p className="price">$29</p>
      <p className="note">
        現在の paid beta 価格は 1 Mac license あたり $29 です。The current paid-beta price is one Mac license for $29.
      </p>
      <p>
        有効な縮小変換 20 回までは無料です。Failed, cancelled, or larger-result conversions do not count toward the trial.
      </p>
      <p>
        有効化した Mac で Pro を解除せずに使えます。Your recordings stay local.
      </p>
      <p>
        DropSquash は recorder や timeline editor ではなく、Mac 画面収録の automatic local post-processor として価格設定しています。
      </p>
      <p className="note">
        public checkout 前は manual beta issuance を許可しています。Public checkout remains blocked until the public site, legal pages, release status, and distribution proof all match the shipped paid-beta app.
      </p>
      <p className="note">live checkout URL は commerce validation 完了後にこのページへ公開します / The live checkout URL will be published here after commerce validation completes.</p>
      <p className="note">
        現在の beta に built-in recording や automatic privacy masking は含みません。
      </p>
      <p className="note">
        Checkout opens after public policy alignment, Lemon Squeezy validation, and release publication are complete.
      </p>
      <ul>
        <li>対応経路 / Current supported path: macOS screen recordings</li>
        <li>入力形式 / Input formats now: MOV, MP4, M4V</li>
        <li>出力 / Output: smaller MP4 after verification</li>
      </ul>
      <p><Link href="/release-status">公開状況 / Release status</Link></p>
      <p><Link href="/terms">利用規約 / Terms of Service</Link></p>
      <p><Link href="/license">ライセンス方針 / License policy</Link></p>
      <p><Link href="/refund">返金方針 / Refund policy</Link></p>
    </div>
  );
}

function renderPrivacy() {
  return (
    <>
      <p>DropSquash は local-first を前提に設計されています / DropSquash is built around a local-first promise.</p>
      <p>
        product-use rules と account-facing terms は <Link href="/terms">利用規約 / Terms of Service</Link> に分けて記載しています。
      </p>
      <ul>
        <li>画面収録の変換はあなたのコンピュータ上で完結します / Screen recording conversion runs locally on your computer.</li>
        <li>圧縮のために media をアップロードしません / DropSquash does not upload media for compression.</li>
        <li>Telemetry は初期状態で無効です / Telemetry is off by default.</li>
        <li>License activation は license key を入力したときだけ Lemon Squeezy と通信します.</li>
        <li>最初の paid beta では updater は無効です / The updater is disabled for the first paid beta.</li>
        <li>検証済み変換と選択ポリシーがない限り originals は保持されます / Original files are kept unless a verified conversion and your chosen policy allow Trash.</li>
        <li>Optional privacy receipts はローカル JSON です / Optional privacy receipts are local JSON files.</li>
      </ul>
    </>
  );
}

function renderTerms() {
  return (
    <>
      <p>このページは current paid beta の利用ルールです / These terms describe the current paid-beta product rules.</p>
      <Section title="製品範囲 / Product scope" text="DropSquash は現在、対応する画面収録をより小さい MP4 にするための local-first macOS app です。It is not yet a recorder or timeline editor." />
      <Section title="ライセンスと認証 / License and activation" text="有料利用には有効なライセンスと認証フローが必要です。Additional activation details are published on the License policy page." />
      <Section title="公開条件 / Release gating" text="paid beta はまだ公開前です。Public download と checkout は checksum publication、release evidence、production URL readiness、commerce validation が揃うまで無効です。" />
      <Section title="利用者の責任 / User responsibility" text="処理する media と、変換後ファイルを共有する先については利用者が責任を持ちます。You remain responsible for what you share." />
      <Section title="サポートと返金 / Support and refund path" text="サポート窓口は Support ページに、返金導線は Refund Policy ページに分けて公開しています。Support and refund are intentionally documented separately." />
      <Section title="ステータス / Status" text="この文書は current public policy として運用します。It may be revised when the paid-beta product or release conditions change." />
    </>
  );
}

function renderSupport() {
  return (
    <>
      <p>
        現時点の DropSquash support は{" "}
        <a href="https://github.com/mt4110/drop-squash/issues">GitHub Issues</a>.
      </p>
      <Section title="動画はアップロードされる？ / Does DropSquash upload my videos?" text="No. DropSquash does not upload media for compression." />
      <Section title="元ファイルは消える？ / Does it delete originals?" text="No. Originals are kept by default." />
      <Section title="失敗変換は trial に数える？ / Does a failed conversion count?" text="いいえ。Failed、cancelled、または smaller にならなかった conversion は、現在の 20 回 beta trial に数えません。" />
      <Section title="今の対応範囲は？ / What is supported right now?" text="現在の公開対象は macOS のみです。MOV / MP4 / M4V の画面収録を、single-file の local post-processing で smaller MP4 にします。" />
      <Section title="command-line media tools を使う？ / Does it use command-line media tools?" text="No. The product media path does not shell out to command-line media tools." />
      <Section title="privacy receipt とは？ / What is a privacy receipt?" text="有効時は、出力ファイルの横に local JSON receipt を書きます。It records uploaded_bytes = 0, metadata_policy = preserve, and file names instead of absolute paths." />
      <Section title="返金方針はどこ？ / Where is the refund policy?" text="The refund policy is published before checkout goes live so the purchase path can be reviewed in advance." />
      <Section title="利用規約はどこ？ / Where are the product-use terms?" text="The Terms of Service are published separately from the license policy." />
      <Section title="変換時の問い合わせ情報 / Conversion help" text="conversion issue のときは、app version、macOS version、input file type、元サイズの目安、selected profile、selected output size、exact error text を添えてください。" />
      <p>
        問い合わせは{" "}
        <a href="https://github.com/mt4110/drop-squash/issues">GitHub Issues</a>{" "}
        を利用してください。The paid beta support address is not published yet.
      </p>
      <p>
        最初のメッセージで画面収録は送らないでください。Share media only after removing private content and only if support explicitly asks for a sample.
      </p>
    </>
  );
}

function renderLicense() {
  return (
    <>
      <p>個人向け Pro license は、有効化した device 上で successful conversions を無制限にします。One activation unlocks ongoing use on that Mac.</p>
      <p>その Mac で一度 Pro を有効化すれば、継続して使えます。Your recordings stay local.</p>
      <p>アプリは license-key fingerprint、instance identifiers、validation timestamps を保持します。raw license key 自体は保持しません。</p>
      <p>
        desktop の forget action はその Mac の local license cache を削除します。Server-side deactivation には raw license key が必要で、自動では行いません。
      </p>
      <p>
        successful validation 後は offline grace を許可します。grace が切れた場合は、一度再接続してその Mac の Pro 状態を更新してください。
      </p>
      <p>
        このページは <Link href="/terms">利用規約 / Terms of Service</Link> の代わりではありません。activation trouble は <Link href="/support">Support</Link> を確認し、refund handling は <Link href="/refund">返金方針 / Refund policy</Link> を参照してください。
      </p>
    </>
  );
}

function renderRefund() {
  return (
    <>
      <p>この返金方針は public checkout 前の paid beta を対象にします / This refund policy covers the pre-checkout paid beta until public checkout goes live.</p>
      <p>DropSquash は、画面収録を安全に軽くする narrow local Mac utility として販売予定です。</p>
      <p>product-use rules は <Link href="/terms">利用規約 / Terms of Service</Link> に分けて記載します。</p>
      <p>DropSquash が activate できない、または supported Mac で基本的な local conversion workflow を実行できない場合は、<Link href="/support">Support</Link> へ連絡してください。</p>
      <p>private / manual paid beta 中の refund request は support review 後に手動対応します。Manual handling remains the current path.</p>
      <p>public checkout 開始後の refund request は Lemon Squeezy order flow を通じて確認します。Until then, no public checkout URL is published.</p>
    </>
  );
}

function renderDownload() {
  return (
    <>
      <p>
        macOS beta は準備中です。Signed、notarized、clean-machine verified artifact は揃っていますが、public download は checksum publication と release publication が整うまで待機します。
      </p>
      <div className="card">
        <h2>現在の状況 / Current status</h2>
        <p>
          local release build では <code>DropSquash.app</code> と{" "}
          <code>DropSquash.dmg</code> を生成できます。Current release evidence already covers Developer ID signing, notarization, stapling, and Gatekeeper verification for the same single-file Mac screen-recording workflow.
        </p>
        <p><Link href="/release-status">公開状況 / Release status</Link></p>
      </div>
      <div className="card">
        <h2>現在の配布対象 / Supported release target</h2>
        <p>
          現在の release target は、<code>DropSquash.dmg</code> として配布する signed and notarized macOS app bundle です。Windows と Linux の実装作業は repository にありますが、public conversion support はまだ claim しません。
        </p>
      </div>
    </>
  );
}

function renderReleaseStatus() {
  return (
    <div className="card">
      <h2>有料ベータはまだ公開前です / Paid beta is not public yet</h2>
      <p>public release には published checksum、manual QA evidence、public web finalization、live checkout validation が必要です。Developer ID signed and notarized <code>DropSquash.dmg</code> と Gatekeeper evidence はすでに揃っています。</p>
      <p>
        Payment onboarding は product truth の後段です。download や checkout を開く前に、public information、legal terms、pricing expectations、distribution evidence が実アプリと一致している必要があります。
      </p>
      <p>
        現在の paid-beta scope は post-processing のみです。DropSquash is the automatic local post-processor for Mac screen recordings. Built-in screen recording と automatic privacy masking は future candidates であり、current release promise ではありません。
      </p>
      <p>
        <code>docs/release-blockers.md</code> の全行が traceable Evidence reference つきで Verified になるまで、download と checkout link は無効のままです。
      </p>
      <p>
        <Link href="/download">ダウンロード状況 / Download status</Link>、<Link href="/pricing">価格と checkout 状況 / Pricing</Link>、<Link href="/privacy">プライバシー / Privacy policy</Link>、<Link href="/terms">利用規約 / Terms of Service</Link>、<Link href="/license">ライセンス方針 / License policy</Link>、<Link href="/support">サポート / Support</Link>、<Link href="/refund">返金方針 / Refund policy</Link> を確認してください。
      </p>
    </div>
  );
}

function Card({ title, text }: { title: string; text: string }) {
  return (
    <article className="card">
      <h2>{title}</h2>
      <p>{text}</p>
    </article>
  );
}

function CopyBlock(props: {
  title: string;
  text: string;
  linkHref?: string;
  linkLabel?: string;
}) {
  return (
    <div className="band-copy">
      <h2>{props.title}</h2>
      <p>{props.text}</p>
      {props.linkHref && props.linkLabel ? (
        <p>
          <Link href={props.linkHref}>{props.linkLabel}</Link>
        </p>
      ) : null}
    </div>
  );
}

function Section({ title, text }: { title: string; text: string }) {
  return (
    <>
      <h2>{title}</h2>
      <p>{text}</p>
    </>
  );
}

function nav(...links: NavLink[]) {
  return links;
}

function link(labelJa: string, labelEn: string, href: string): NavLink {
  return { href, labelJa, labelEn };
}
