# DropSquash Rust Repository Blueprint

Rust 製の新規リポジトリとして、画面録画圧縮・共有用ローカルアプリを作るための設計書です。

2026-07-10 revision:

- macOS / Windows / Linux を同じ product core で支える。
- `ffmpeg` / `ffprobe` の実行、リンク、同梱を行わない。
- Rust から各 OS のネイティブ media API を呼び、利用可能なら hardware codec を使う。
- Nix は再現可能な開発・CI環境に限定し、製品ランタイムには含めない。

このドキュメントは Codex / RunAlways にそのまま渡せる粒度を目指しています。

---

## 0. 結論

別リポジトリ化は賛成です。

ただし、旧 `rec-watch` を単純に Rust へ移植するのではなく、次のように定義し直します。

> 巨大な画面録画を、ローカルだけで共有可能なサイズへ変換する Drop Zone。

推奨名は以下です。

| 項目 | 推奨値 |
|---|---|
| Product name | **DropSquash** |
| Repository | `dropsquash` |
| CLI command | `dropsquash` |
| Desktop app | `DropSquash.app` |
| Homebrew formula | `dropsquash` |
| Homebrew cask | `dropsquash` |
| Pro edition | `DropSquash Pro` |
| Tagline | `Drop huge screen recordings. Squash them locally.` |

重要: これは商標調査ではありません。公開前に GitHub / crates.io / Homebrew / domain / Apple bundle identifier / 商標を最終確認してください。

---

## 1. 名前設計

### 1.1 第一候補: DropSquash

理由:

- `Drop`: D&D の体験と一致する。
- `Squash`: 圧縮する、潰して小さくする、という意味が直感的。
- 動画に限定しすぎないため、将来「音声抽出」「議事録 hook」「メタデータ削除」へ広げても破綻しにくい。
- CLI 名として短い。
- 販売ページで説明しやすい。

### 1.2 表記ルール

| 用途 | 表記 |
|---|---|
| アプリ名 | DropSquash |
| GitHub repo | `dropsquash` |
| CLI | `dropsquash` |
| Rust package prefix | `dropsquash-*` |
| config dir | `DropSquash` |
| macOS app id, provisional | `io.github.mt4110.dropsquash` |
| macOS app id, final | `com.<owned-domain>.dropsquash` |

`io.github.mt4110.dropsquash` は仮の bundle identifier です。販売前に所有ドメインがあるなら `com.<owned-domain>.dropsquash` に移行してください。

### 1.3 避ける名前

以下は軽い Web 調査上、競合・近似・既存ツールが見えたため避けます。

| 名前 | 避ける理由 |
|---|---|
| `RecDrop` | 既存の録音系アプリと近い。 |
| `DropMov` / `drop.mov` | 既存の動画・ファイル共有サービスと近い。 |
| `ClipForge` | GitHub 上に動画・クリップ系プロジェクトが複数ある。 |
| `ShrinkDrop` | 既存の圧縮系サービス名と近い。 |
| `ClipPrune` | 既存の古い Windows clipboard utility 名と衝突する。 |
| `RecShrink` | 別業界の既存製品名と近い。 |
| `RecLite` / `ScreenLite` | 既存アプリ・ツールと近い。 |

---

## 2. プロダクト定義

### 2.1 作るもの

DropSquash は、macOS / Windows / Linux の画面録画をローカル環境だけで小さな共有用 `.mp4` へ変換するアプリです。

共通入力 baseline は H.264/AAC を格納した `.mov`, `.mp4`, `.m4v` とします。MKV / WebM などは backend の capability が確認できた OS だけで表示し、未対応形式を UI に固定表示しません。

主な体験:

```text
録画ファイルを落とす
  ↓
ファイルが書き終わるまで安全に待つ
  ↓
用途に応じて圧縮する
  ↓
圧縮結果を表示する
  ↓
必要なら原本を安全に処理する
```

### 2.2 作らないもの

v1 では以下を作りません。

- 本格的な動画編集ソフト。
- タイムライン編集。
- クラウドアップロードサービス。
- 共有リンク生成サービス。
- 自前 H.264 / HEVC エンコーダ。
- 外部 media CLI の起動。
- codec が利用できない環境での暗黙の software fallback。
- 画面録画そのものの実行機能。
- 議事録 AI を本体に内蔵すること。

議事録・Whisper・LLM は、後段 hook として外部プロセス連携にします。

---

## 3. リポジトリ戦略

### 3.1 推奨

最初は **private monorepo** として `mt4110/dropsquash` を作ります。

理由:

- 販売導線、ライセンス処理、UI、署名、公証を含むため、最初から public にするとライセンス回避や販売実験のノイズが増える。
- まだプロダクト仮説検証前なので、OSS コミュニティ対応より実装速度を優先する。
- v1 完成後に `dropsquash-core` を public OSS 化する方が安全。

### 3.2 将来の分離案

```text
mt4110/dropsquash          private: full product, desktop app, sales integration
mt4110/dropsquash-core     public: core crates, CLI, fileguard, queue, encoder traits
mt4110/homebrew-tap        public: formula/cask
```

この分離にすると、OSS 信頼と商用保護を両立できます。

---

## 4. 最終目標値

これは v1.0 の完成条件です。測定できる形にします。

### 4.1 UX 目標

| 項目 | 目標 |
|---|---|
| 初回起動 | 30 秒以内に D&D 変換を開始できる |
| 日常操作 | ファイル D&D だけで変換完了 |
| 設定操作 | 出力先、用途プロファイル、解像度、原本処理だけ |
| UI | 小さな Drop Zone + 設定 drawer |
| 通知 | 変換完了、失敗、原本処理結果を通知 |
| 失敗時 | 原本を絶対に移動しない |

### 4.2 圧縮目標

| 入力タイプ | 目標 |
|---|---|
| 画面録画、静的 UI 多め | 80% - 96% のサイズ削減 |
| 画面録画、操作多め | 70% - 90% のサイズ削減 |
| カメラ動画、動き多め | 40% - 75% のサイズ削減 |
| 文字中心の録画 | 文字可読性を最優先 |
| 出力が入力より大きい場合 | 失敗扱い、または keep 扱い |

圧縮率は保証値ではなく目標値です。入力内容で大きく変わります。

### 4.3 性能目標

| 項目 | 最低ライン | 目標ライン |
|---|---:|---:|
| Hardware codec 対応環境の 1080p 画面録画 | 実時間以上 | 2x - 5x 実時間 |
| Hardware codec 非対応環境 | 明示的に未対応を表示 | 勝手に低速 fallback しない |
| キュー処理 | 連続 100 件でクラッシュ 0 | 連続 500 件でクラッシュ 0 |
| UI 応答 | 変換中も操作可能 | 100ms 以内に主要操作反応 |
| ファイル検知 | 固定 sleep 不使用 | stable wait + timeout |
| 失敗時原本移動 | 0 件 | 0 件 |

### 4.4 プライバシー目標

| 項目 | 目標 |
|---|---|
| 動画アップロード | 0 |
| デフォルト telemetry | なし |
| ライセンス通信 | ライセンス検証時のみ |
| 変換履歴 | ローカル保存 |
| Privacy Receipt | 変換結果のローカル JSON / Markdown 出力 |

### 4.5 販売目標

| 項目 | 目標 |
|---|---|
| Trial | 成功変換 10 回まで無料 |
| 失敗変換 | カウントしない |
| Paywall | 削減容量、平均圧縮率、推定節約時間を表示 |
| 販売 | Lemon Squeezy などの外部 checkout |
| ライセンス保存 | OS Keychain / Credential Manager |
| 価格初期案 | Personal lifetime $19 - $29 |

---

## 5. 全体アーキテクチャ

### 5.1 基本方針

- Rust workspace で core / CLI / desktop app を分離する。
- UI は Tauri v2。
- 動画 probe / encode は trait 化し、OS 固有実装を Rust crate 内に隔離する。
- macOS は AVFoundation + VideoToolbox、Windows は Media Foundation、Linux は `gstreamer-rs` 経由の GStreamer を使用する。
- 外部 media executable、shell、PATH lookup は変換経路で使用しない。
- runtime capability probe を行い、利用できない codec を成功可能として表示しない。
- 原本削除やゴミ箱移動は encoder から完全分離する。
- Trial / license は conversion success result をもとに判定する。
- すべての重要処理をテスト可能な crate に閉じ込める。

### 5.2 Platform backend matrix

| OS | Probe / demux / mux | Video encode | Product rule |
|---|---|---|---|
| macOS | AVFoundation | VideoToolbox | Hardware H.264 を第一候補にする |
| Windows | Media Foundation Source Reader / Sink Writer | Hardware MFT | `MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS` を要求する |
| Linux | GStreamer through `gstreamer-rs` | Allowlisted hardware element | `gst-libav` を使わず、plugin discovery を製品同梱範囲へ固定する |

Rust は orchestration、安全判定、設定、履歴、IPC を担います。codec の速度は Rust そのものではなく hardware backend によって得ます。従って販売表現は「pure Rust だから高速」ではなく、次を使います。

```text
Native hardware acceleration.
No external ffmpeg process.
Local-only conversion.
```

Linux は driver と配布形式による差が最も大きいため、capability probe を通らない環境では理由と不足 component を表示します。未監査の plugin や software codec を自動で取得しません。

### 5.3 データフロー

```text
Drop / Watch event
  ↓
fileguard::wait_until_stable
  ↓
media::probe
  ↓
profiles::resolve
  ↓
queue::enqueue
  ↓
encoder::encode
  ↓
verifier::verify_output
  ↓
history::record_success / record_failure
  ↓
postprocess::handle_source
  ↓
license::record_successful_conversion
  ↓
platform::notify
```

### 5.4 安全原則

原本を移動できる条件:

```text
conversion succeeded
AND output exists
AND output size > 0
AND original size > 0
AND output size < original size
AND source policy permits moving
```

この条件を満たさない場合、原本は必ず保持します。

### 5.5 Security boundary

- 入力動画、container metadata、file name をすべて untrusted data として扱う。
- WebView から encoder や OS API を直接呼ばず、型付き Tauri command だけを公開する。
- encoder は shell command を組み立てず、外部 executable を探索しない。
- FFI / `unsafe` は OS backend module に限定し、core crate へ漏らさない。
- output は一時ファイルへ書き、native probe とサイズ検証に成功してから atomic rename する。
- encoder crash と resource exhaustion を UI process から分離する worker 化を release 前の必須条件にする。
- worker は network capability を持たず、選択された入力と出力 directory 以外へアクセスしない。
- Linux の plugin registry と探索 path はアプリ管理下へ固定し、ユーザー環境から任意 plugin を注入させない。

---

## 6. Repository Tree

```text
dropsquash/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── desktop-ci.yml
│   │   ├── release.yml
│   │   ├── nightly.yml
│   │   └── security.yml
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   ├── feature_request.yml
│   │   └── conversion_failure.yml
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── CODEOWNERS
│   └── dependabot.yml
│
├── apps/
│   ├── cli/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   │
│   └── desktop/
│       ├── README.md
│       ├── package.json
│       ├── pnpm-lock.yaml
│       ├── src-tauri/
│       │   ├── Cargo.toml
│       │   ├── tauri.conf.json
│       │   ├── capabilities/
│       │   │   └── default.json
│       │   ├── icons/
│       │   └── src/
│       │       ├── main.rs
│       │       ├── commands.rs
│       │       ├── state.rs
│       │       └── events.rs
│       │
│       └── web/
│           ├── index.html
│           ├── package.json
│           ├── src/
│           │   ├── main.tsx
│           │   ├── App.tsx
│           │   ├── components/
│           │   │   ├── DropZone.tsx
│           │   │   ├── QueuePanel.tsx
│           │   │   ├── ResultCard.tsx
│           │   │   ├── TrialBanner.tsx
│           │   │   └── SettingsDrawer.tsx
│           │   └── lib/
│           │       ├── commands.ts
│           │       └── format.ts
│           └── vite.config.ts
│
├── crates/
│   ├── dropsquash-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs
│   │       ├── paths.rs
│   │       ├── config.rs
│   │       ├── job.rs
│   │       └── result.rs
│   │
│   ├── dropsquash-fileguard/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── stable.rs
│   │
│   ├── dropsquash-media/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── probe.rs
│   │       └── types.rs
│   │
│   ├── dropsquash-profiles/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── auto.rs
│   │       ├── slack.rs
│   │       ├── docs.rs
│   │       ├── archive.rs
│   │       └── privacy.rs
│   │
│   ├── dropsquash-encoder/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── backend.rs
│   │       ├── verify.rs
│   │       ├── videotoolbox.rs
│   │       ├── media_foundation.rs
│   │       └── gstreamer.rs
│   │
│   ├── dropsquash-queue/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── store.rs
│   │       ├── worker.rs
│   │       └── events.rs
│   │
│   ├── dropsquash-postprocess/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── policy.rs
│   │       └── decision.rs
│   │
│   ├── dropsquash-history/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── jsonl.rs
│   │       └── metrics.rs
│   │
│   ├── dropsquash-license/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── trial.rs
│   │       ├── provider.rs
│   │       ├── lemonsqueezy.rs
│   │       └── cache.rs
│   │
│   ├── dropsquash-platform/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── notify.rs
│   │       ├── trash.rs
│   │       ├── dialog.rs
│   │       ├── keychain.rs
│   │       └── watch.rs
│   │
│   └── dropsquash-privacy/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── metadata.rs
│           └── receipt.rs
│
├── docs/
│   ├── architecture.md
│   ├── native-backends.md
│   ├── product.md
│   ├── licensing.md
│   ├── release.md
│   ├── reproducible-builds.md
│   ├── privacy.md
│   └── troubleshooting.md
│
├── packaging/
│   ├── homebrew/
│   │   ├── Formula/dropsquash.rb
│   │   └── Casks/dropsquash.rb
│   ├── macos/
│   │   ├── entitlements.plist
│   │   └── dmg-background.png
│   ├── windows/
│   │   └── winget.yaml
│   └── linux/
│       └── flatpak/
│
├── xtask/
│   ├── Cargo.toml
│   └── src/main.rs
│
├── tests/
│   ├── fixtures/
│   └── integration/
│
├── website/
│   ├── README.md
│   └── landing-copy.md
│
├── .gitignore
├── flake.nix
├── flake.lock
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── deny.toml
├── README.md
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
└── CHANGELOG.md
```

---

## 7. Crate Responsibilities

### 7.1 `dropsquash-core`

責務:

- domain types
- error type
- config schema
- job/result model
- application paths

禁止:

- UI 依存
- Tauri 依存
- media executable の起動
- OS dialog 直接実行

### 7.2 `dropsquash-fileguard`

責務:

- 録画ファイルが書き終わったか判定する。
- ESET / Defender / クラウド同期 / 録画アプリの遅延に耐える。

API 案:

```rust
pub struct StabilityOptions {
    pub timeout: Duration,
    pub interval: Duration,
    pub stable_samples: usize,
}

pub struct StabilityResult {
    pub path: PathBuf,
    pub size: u64,
    pub modified_at: SystemTime,
    pub checked_at: SystemTime,
    pub sample_count: usize,
}

pub async fn wait_until_stable(
    path: &Path,
    options: StabilityOptions,
    cancel: CancellationToken,
) -> Result<StabilityResult>;
```

判定条件:

```text
stat 成功
AND size が stable_samples 回連続で不変
AND modified_at が stable_samples 回連続で不変
AND read open 成功
AND timeout していない
```

### 7.3 `dropsquash-encoder`

責務:

- encoder backend trait
- OS-native backend
- 出力検証
- runtime capability probe
- OS API / FFI の隔離

API 案:

```rust
pub trait EncoderBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn probe_capabilities(&self) -> Result<EncoderCapabilities>;
    async fn encode(&self, job: EncodeJob) -> Result<EncodeResult>;
}
```

Product backends:

```text
VideoToolboxEncoder       macOS
MediaFoundationEncoder    Windows
GStreamerEncoder          Linux
```

全 backend の共通ルール:

```text
no shell
no PATH lookup
no external media executable
hardware codec preferred
no silent fallback
```

`probe_capabilities` は compile-time の想定値ではなく、その端末で実際に利用可能な codec、hardware acceleration、container を返します。Linux backend は `gst-libav` を拒否し、製品で監査した element のみを pipeline に組み込みます。

### 7.4 `dropsquash-postprocess`

責務:

- 原本処理ポリシー
- ゴミ箱移動判定
- 確認ダイアログ前の安全判定

Source policy:

```text
keep
trash
ask
archive
```

v1 では `keep`, `trash`, `ask` まででよいです。

### 7.5 `dropsquash-license`

責務:

- 成功変換回数のカウント
- trial 状態
- license provider trait
- Lemon Squeezy provider
- local license cache

禁止:

- API secret をアプリに埋め込む。
- 失敗変換を trial 消費に含める。
- ライセンス検証失敗でローカルファイル処理を破壊する。

### 7.6 `dropsquash-history`

責務:

- 成功/失敗履歴の永続化
- paywall で表示する削減量計算
- privacy receipt 出力

保存先:

```text
macOS: ~/Library/Application Support/DropSquash/history.jsonl
```

将来 SQLite に移行してもよいですが、v0.1 は JSONL で十分です。

---

## 8. UI Spec

### 8.1 Main Window

```text
┌────────────────────────────────────┐
│              DropSquash             │
│                                    │
│          Drop recording here        │
│      Supported formats on device     │
│                                    │
├────────────────────────────────────┤
│ Output    ~/Movies/DropSquash       │
│ Profile   Auto                      │
│ Size      Auto / 1080p / 720p       │
│ Source    Ask after success         │
│ Privacy   Local only                │
└────────────────────────────────────┘
```

### 8.2 Result Card

```text
Done

Original: 104.2 MB
Output:     4.8 MB
Saved:     95.4%

[Open Output] [Move Original to Trash] [Keep Original]
```

### 8.3 Trial Paywall

```text
Trial complete

You converted 10 recordings.
Total saved: 3.8 GB
Average reduction: 91.2%
Estimated upload time saved: 42 min

[Buy DropSquash Pro] [Enter License Key]
```

### 8.4 Settings

```text
Output folder
Default profile
Resolution mode
Source policy
Watch folders
License
Privacy receipt
Advanced encoder backend
```

### 8.5 UI 非目標

- タイムライン編集を入れない。
- 圧縮パラメータを前面に出しすぎない。
- 画面を大きくしない。
- クラウド連携を v1 に入れない。

---

## 9. Profiles

### 9.1 初期プロファイル

| Profile | 目的 |
|---|---|
| Auto | 入力を見て自動選択 |
| Slack | 小ささ優先 |
| Docs | 文字可読性優先 |
| Archive | 保存用 |
| Privacy | メタデータ削除、音声削除オプション |

### 9.2 Auto 判定

MVP では簡単でよいです。

```text
if width > 1920 => downscale candidate
if duration is long and video is screen-like => lower fps candidate
if audio absent => skip audio stream
if output would be larger => fail/keep
```

高度な scene analysis は v1.2 以降。

---

## 10. Licensing and Sales

### 10.1 Trial policy

```text
success_conversion_count < 10 => Free
success_conversion_count >= 10 AND no valid license => Locked
valid license => Pro
```

成功変換の定義:

```text
encoder returned success
AND output exists
AND output size > 0
AND output size < original size
```

失敗変換はカウントしません。

### 10.2 License flow

```text
User clicks Buy
  ↓
Open checkout URL
  ↓
User receives license key
  ↓
User enters key in app
  ↓
LicenseProvider.activate(key, instance)
  ↓
Store license state in OS keychain
  ↓
Unlock Pro
```

### 10.3 Secrets rule

絶対にやらないこと:

```text
- Lemon Squeezy API secret を Tauri アプリに埋め込む
- GitHub secret を repo に commit する
- 署名用 private key を repo に commit する
- license bypass flag を release build に残す
```

### 10.4 Pricing draft

| Plan | Draft price |
|---|---:|
| Personal Lifetime | $19 - $29 |
| Team Lifetime | $49 - $99 |
| Business | $149+ |

最初はサブスクではなく買い切り推奨です。

---

## 11. GitHub Repository Settings

### 11.1 Repository

推奨初期設定:

```text
Visibility: Private
Default branch: main
Issues: enabled
Projects: enabled
Discussions: disabled initially
Wiki: disabled
Merge commits: disabled
Squash merge: enabled
Rebase merge: optional
Auto-delete merged branches: enabled
```

### 11.2 Branch protection

`main` に対して以下を設定します。

```text
Require pull request before merging: true
Require approvals: 1
Require status checks: true
Require branches to be up to date: true
Require conversation resolution: true
Require signed commits: optional
Require linear history: true
Allow force pushes: false
Allow deletions: false
```

Required checks:

```text
ci / fmt
ci / clippy
ci / test
ci / cargo-deny
ci / desktop-build
```

### 11.3 Labels

```text
kind/bug
kind/feature
kind/refactor
kind/docs
kind/security
kind/release
area/core
area/desktop
area/cli
area/encoder
area/license
area/packaging
area/ui
priority/p0
priority/p1
priority/p2
status/blocked
status/needs-design
status/ready
```

### 11.4 Milestones

```text
v0.1 Core Scaffold
v0.2 Native backend contracts
v0.3 macOS VideoToolbox backend
v0.4 Windows Media Foundation backend
v0.5 Linux GStreamer backend
v0.6 Desktop Drop Zone + Trial + History
v0.7 Signed cross-platform beta
v1.0 Cross-platform launch
```

### 11.5 GitHub Secrets

手動で入れる secrets:

```text
APPLE_ID
APPLE_PASSWORD
APPLE_TEAM_ID
APPLE_SIGNING_IDENTITY
APPLE_CERTIFICATE_P12
APPLE_CERTIFICATE_PASSWORD
KEYCHAIN_PASSWORD
TAURI_SIGNING_PRIVATE_KEY
TAURI_SIGNING_PRIVATE_KEY_PASSWORD
LEMONSQUEEZY_CHECKOUT_URL
LEMONSQUEEZY_STORE_ID
LEMONSQUEEZY_PRODUCT_ID
LEMONSQUEEZY_VARIANT_ID
LEMONSQUEEZY_WEBHOOK_SECRET
```

注意:

- `LEMONSQUEEZY_API_KEY` をクライアントアプリに渡さない。
- GitHub Actions 内で販売サイト同期や webhook テストに使う場合のみ secret として保持する。
- `TAURI_SIGNING_PRIVATE_KEY` は漏洩したらアップデート信頼が壊れる。

### 11.6 Environments

```text
staging
production
```

`production` は required reviewer を設定します。

---

## 12. GitHub Actions

### 12.1 `ci.yml`

実行内容:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

### 12.2 `desktop-ci.yml`

実行内容:

```text
pnpm install
pnpm lint
pnpm test
cargo test -p dropsquash-desktop
cargo tauri build --debug on macOS, Windows, and Linux runners
```

### 12.3 `release.yml`

trigger:

```text
push tag v*.*.*
```

実行内容:

```text
build unsigned payloads on macOS, Windows, and Linux
sign and notarize macOS DMG
sign Windows installer
build Linux Flatpak from an approved runtime
sign updater artifacts per platform
upload GitHub Release draft
write checksums
```

Windows installer signing and Linux package generation are separate jobs. OS-native backend tests must run on their native runner; cross-compilation alone is not accepted as verification.

### 12.4 `nightly.yml`

目的:

```text
main の nightly build を作る
販売用ではなく検証用
```

### 12.5 `security.yml`

実行内容:

```text
cargo audit
cargo deny
cargo machete
npm audit, if used
```

### 12.6 Nix policy

Nix は開発環境と依存 pin に使いますが、DropSquash の利用条件にはしません。

```text
flake.lock is committed
nix develop supports Apple Silicon macOS and Linux development
Windows native build/test runs on a Windows runner
release artifacts contain no /nix/store reference
Nix is never installed by the app
```

Nix による再現性は unsigned payload までを対象にします。code signing、notarization、timestamp 付き installer は秘密鍵と外部 timestamp service を使うため、再現可能 build と分離して検証します。

---

## 13. Homebrew Strategy

### 13.1 Repositories

```text
mt4110/dropsquash       product repo
mt4110/homebrew-tap     formula/cask repo
```

### 13.2 Commands

CLI:

```bash
brew install mt4110/tap/dropsquash
```

Desktop:

```bash
brew install --cask mt4110/tap/dropsquash
```

### 13.3 配布方針

- CLI は formula。
- Desktop app は cask。
- Pro unlock はアプリ内 license key。
- Homebrew では license key を扱わない。

---

## 14. Codex RunAlways Prompt: Phase 0

最初に Codex へ渡すプロンプトです。

```text
Create a new Rust/Tauri repository scaffold for a product named DropSquash.

Goal:
Build the initial private monorepo scaffold for a local-first screen recording compression app.
Do not implement full video encoding yet. Do not implement payment yet. Do not embed any secrets. Do not add a cloud upload feature.

Repository name:
dropsquash

Product:
DropSquash — Drop huge screen recordings. Squash them locally.

Architecture:
Use a Cargo workspace with apps/cli, apps/desktop, and crates/*.
Use Tauri v2 for the desktop app, but keep all business logic out of the Tauri command layer.
The core must be testable without Tauri, native media frameworks, OS dialogs, or network access.

Implement the following scaffold:

1. Cargo workspace
- apps/cli
- apps/desktop/src-tauri
- crates/dropsquash-core
- crates/dropsquash-fileguard
- crates/dropsquash-media
- crates/dropsquash-profiles
- crates/dropsquash-encoder
- crates/dropsquash-queue
- crates/dropsquash-postprocess
- crates/dropsquash-history
- crates/dropsquash-license
- crates/dropsquash-platform
- crates/dropsquash-privacy
- xtask

2. Core domain types
- EncodeJob
- EncodeResult
- MediaInfo
- Profile
- SourcePolicy
- LicenseState
- TrialState
- AppConfig
- AppError

3. Fileguard
- Implement wait_until_stable with deterministic tests using temp files.
- No fixed sleep assumptions in production API.
- No mutation of source files.

4. Postprocess safety
- Implement source handling decision logic.
- Never allow original movement unless conversion succeeded, output exists, output size > 0, original size > 0, and output size < original size.
- Unit tests for all safety cases.

5. History
- Implement JSONL writer/reader for conversion results.
- Implement aggregate metrics: successful conversion count, total original bytes, total output bytes, saved bytes, average reduction percent.
- Unit tests.

6. License trial logic
- Implement local trial counting based only on successful conversions.
- Trial limit should be a named config value, not a magic number inside logic.
- No Lemon Squeezy network call in Phase 0.
- Unit tests.

7. Encoder trait
- Define EncoderBackend trait.
- Add NoopEncoder for tests.
- Add VideoToolboxEncoder, MediaFoundationEncoder, and GStreamerEncoder capability stubs.
- Do not execute, link, download, or bundle ffmpeg/ffprobe.

8. CLI
- Add a minimal CLI using clap:
  - dropsquash convert <input> --output-dir <dir> --profile auto|slack|docs|archive|privacy
  - dropsquash stats
  - dropsquash doctor
- CLI can use NoopEncoder behind a feature flag for tests.

9. Desktop app scaffold
- Create a minimal Tauri app with a small Drop Zone UI.
- The UI may call placeholder commands only.
- Do not implement full app signing or updater in this phase.

10. GitHub files
- README.md
- SECURITY.md
- CONTRIBUTING.md
- CHANGELOG.md
- .github/workflows/ci.yml
- .github/dependabot.yml
- .github/PULL_REQUEST_TEMPLATE.md
- .github/ISSUE_TEMPLATE/bug_report.yml
- .github/ISSUE_TEMPLATE/feature_request.yml
- .github/CODEOWNERS

11. Quality gates
- cargo fmt --all -- --check passes
- cargo clippy --workspace --all-targets -- -D warnings passes
- cargo test --workspace passes

Acceptance criteria:
- The repository compiles.
- Tests pass without native media frameworks, Tauri runtime, OS dialogs, or network access.
- No secrets or private keys are committed.
- No payment integration is implemented yet.
- Business logic is not inside UI code.
- Source deletion/move safety is covered by tests.

After implementation, summarize:
- Files created
- Commands run
- Test results
- Remaining risks
- Recommended Phase 1 tasks
```

---

## 15. Codex RunAlways Prompt: Phase 1

Phase 0 完了後に渡します。

```text
Implement the native media foundation for DropSquash CLI.

Do not add payment. Do not require signed builds.

Implement:
- VideoToolboxEncoder on macOS through AVFoundation and VideoToolbox bindings.
- MediaFoundationEncoder on Windows through Windows bindings.
- GStreamerEncoder on Linux through gstreamer-rs with an explicit plugin allowlist.
- Native media probe for each platform.
- convert command end-to-end.
- output verifier.
- history recording.
- stats command.
- doctor command that reports actual native codec and hardware capabilities.
- profile to backend-neutral encoding settings mapping for auto/slack/docs/archive/privacy.
- source policy keep|ask|trash, but ask should not block tests.

Rules:
- Never execute, link, download, or bundle ffmpeg/ffprobe.
- Never use shell or PATH lookup in the media path.
- Never claim a codec is available before runtime capability probing.
- Never load gst-libav or an unapproved GStreamer plugin.
- Never move source if output is missing, zero size, larger than input, or conversion failed.
- Platform-independent tests use NoopEncoder.
- Native smoke tests run on each native OS runner.
- Add integration test with NoopEncoder.

Acceptance:
- cargo fmt, clippy, test pass.
- Manual command documented in README.
```

---

## 16. Codex RunAlways Prompt: Phase 2

```text
Implement the DropSquash desktop Drop Zone MVP.

Use the existing core crates. Do not duplicate business logic in Tauri commands.

Implement:
- Drag and drop file selection.
- Output folder selection.
- Profile selector.
- Resolution selector.
- Source policy selector.
- Conversion queue display.
- Result card showing original size, output size, saved percent.
- Trial banner based on local history/trial state.
- Open output folder button.

Do not implement Lemon Squeezy yet.
Do not implement auto-updater yet.
Do not duplicate native backend logic in Tauri commands.

Acceptance:
- Desktop app builds in debug.
- UI can run a conversion through core service.
- UI remains responsive while conversion runs.
- Failed conversion does not consume trial count.
```

---

## 17. Codex RunAlways Prompt: Phase 3

```text
Implement DropSquash license activation.

Use Lemon Squeezy License API through a LicenseProvider trait.
Do not embed Lemon Squeezy API secrets in the client.
Use only license-key based activation/validation endpoints that are safe for client use.
Store activated license state in OS keychain through dropsquash-platform.

Implement:
- License dialog
- Enter license key
- Activate license
- Validate license
- Deactivate license
- Local cache for UX
- Trial limit lock after successful conversions only
- Buy button that opens checkout URL from build-time public config

Rules:
- Failed conversions do not count.
- License network failure should display a clear error, not crash.
- Do not store license key in plain text files.
- Unit tests use MockLicenseProvider.

Acceptance:
- cargo tests pass without network.
- Manual license flow documented.
```

---

## 18. Codex RunAlways Prompt: Phase 4

```text
Implement cross-platform release packaging for DropSquash.

Implement:
- GitHub Actions release workflow on v* tags.
- Tauri production build.
- Apple code signing, notarization, and DMG artifact.
- Windows code signing and installer artifact.
- Linux Flatpak artifact.
- Tauri updater artifact signing.
- SHA256 checksums.
- Draft GitHub Release upload.

Rules:
- No signing keys in repository.
- All secrets come from GitHub Actions secrets.
- Release workflow should fail closed if signing credentials are missing.
- Do not publish automatically to production without manual approval environment.

Acceptance:
- Workflow syntax validates.
- Unsigned local build path documented.
- Signed release path documented.
```

---

## 19. Initial GitHub Issues

Create these issues immediately after repository creation.

### Issue 1: Scaffold Rust workspace

Labels:

```text
area/core, priority/p0, status/ready
```

Deliverables:

```text
Cargo workspace
all crates created
CI passing
README skeleton
```

### Issue 2: Implement fileguard stable wait

Labels:

```text
area/core, priority/p0, kind/feature
```

Deliverables:

```text
wait_until_stable
unit tests
timeout/cancel support
```

### Issue 3: Implement source postprocess safety

Labels:

```text
area/core, priority/p0, kind/feature
```

Deliverables:

```text
source policy
safety gates
unit tests for every failure case
```

### Issue 4: Implement history JSONL and trial metrics

Labels:

```text
area/license, area/core, priority/p1
```

Deliverables:

```text
history writer/reader
aggregate metrics
trial counter
```

### Issue 5: Implement native backend contracts and CLI

Labels:

```text
area/cli, priority/p1
```

Deliverables:

```text
convert
stats
doctor
profile mapping
runtime capability report
```

### Issue 6: Implement Tauri Drop Zone MVP

Labels:

```text
area/desktop, area/ui, priority/p1
```

Deliverables:

```text
drag and drop
profile selector
output selector
result card
```

### Issue 7: Implement license activation

Labels:

```text
area/license, priority/p1
```

Deliverables:

```text
license dialog
mock provider tests
Lemon provider
keychain storage
```

### Issue 8: Implement release pipeline and macOS packaging

Labels:

```text
area/packaging, priority/p1
```

Deliverables:

```text
signed app
notarized dmg
updater artifact signing
release workflow
```

### Issue 9: Implement Windows Media Foundation backend

Labels:

```text
area/encoder, area/packaging, priority/p1
```

Deliverables:

```text
native probe
hardware MFT encode
Windows runner integration tests
signed installer path
```

### Issue 10: Implement Linux GStreamer backend

Labels:

```text
area/encoder, area/packaging, priority/p1, kind/security
```

Deliverables:

```text
gstreamer-rs pipeline
plugin allowlist
gst-libav rejection test
Flatpak package path
```

### Issue 11: Isolate the native media worker

Labels:

```text
area/core, area/encoder, priority/p0, kind/security
```

Deliverables:

```text
versioned typed IPC
no shell and no network capability
access limited to selected input/output locations
time, memory, and output-size limits
worker crash and cancellation recovery tests
platform sandbox verification
```

---

## 20. README First Draft

```markdown
# DropSquash

Drop huge screen recordings. Squash them locally.

DropSquash turns large screen recordings into small, shareable MP4 files on macOS, Windows, and Linux without uploading your videos to the cloud.

## Why

Screen recordings are often too large for Slack, GitHub, Notion, email, or VPN-heavy environments. DropSquash gives you a tiny local Drop Zone: drag a MOV in, get a compressed MP4 out.

## Principles

- Local-first
- No cloud upload
- Safe original handling
- Screen-recording aware compression
- Small UI
- Scriptable core

## Status

Early private development.
```

---

## 21. Landing Page Copy

```text
Huge screen recordings in.
Tiny shareable MP4s out.
No cloud. No waiting. No leaks.
```

```text
Drop a MOV.
Get a tiny MP4.
Everything stays on your device.
```

```text
Built for people who record screens all day and hate uploading 500MB files through corporate VPNs.
```

---

## 22. Hard Rules

Codex に必ず守らせるルールです。

```text
- Do not implement a custom video codec.
- Do not execute, link, download, or bundle ffmpeg/ffprobe.
- Do not use shell or PATH lookup in the media pipeline.
- Do not load gst-libav or unapproved dynamic media plugins.
- Do not move originals inside encoder code.
- Do not count failed conversions as trial usage.
- Do not store license keys in plain text.
- Do not commit secrets.
- Do not put business logic in React components.
- Do not use fixed sleeps for file completion detection.
- Do not add cloud upload features in v1.
- Do not make UI larger than necessary.
```

---

## 23. Suggested Local Commands

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Tauri:

```bash
cd apps/desktop
pnpm install
pnpm tauri dev
```

---

## 24. Repository Creation Commands

手元で実行する場合の例です。

```bash
gh repo create mt4110/dropsquash \
  --private \
  --description "Drop huge screen recordings. Squash them locally." \
  --clone

cd dropsquash
```

GitHub 設定例:

```bash
gh repo edit mt4110/dropsquash \
  --enable-issues=true \
  --enable-projects=true \
  --enable-wiki=false \
  --delete-branch-on-merge=true
```

Secrets は値を直接履歴に残さず、対話または標準入力で設定します。

```bash
gh secret set APPLE_ID
gh secret set APPLE_PASSWORD
gh secret set APPLE_TEAM_ID
gh secret set TAURI_SIGNING_PRIVATE_KEY
gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD
gh secret set LEMONSQUEEZY_CHECKOUT_URL
```

---

## 25. Final Recommendation

最初の公開形態はこれが安全です。

```text
Repo: private mt4110/dropsquash
Name: DropSquash
Platforms: macOS, Windows, Linux
Distribution: signed DMG, signed Windows installer, Flatpak
Trial: 10 successful conversions
License: Lemon Squeezy
Core: Rust workspace
UI: Tauri v2 Drop Zone
macOS backend: AVFoundation + VideoToolbox
Windows backend: Media Foundation
Linux backend: allowlisted GStreamer through gstreamer-rs
Build environment: Nix for macOS/Linux development only
```

「Rustだけで codec を再実装する」ことはしません。

```text
Rustで壊れないパイプラインを作る。
動画処理はOSのhardware backendをRustから直接制御する。
外部media CLIには依存しない。
```

これが長期的に破綻しにくいです。
