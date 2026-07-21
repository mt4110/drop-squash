import AppKit

private struct FixtureTruth: Codable {
  let frameWidth: Int
  let frameHeight: Int
  let sensitive: [FixtureRect]
  let timedSensitive: [TimedFixtureRect]
}

private struct TimedFixtureRect: Codable {
  let startNs: UInt64
  var endNs: UInt64
  let rect: FixtureRect
}

private struct FixtureRect: Codable {
  let x: Int
  let y: Int
  let width: Int
  let height: Int
}

final class CanvasSecretView: NSView {
  private let text = "Canvas token: dsq_canvas_secret_31QX"
  private let attributes: [NSAttributedString.Key: Any] = [
    .font: NSFont.monospacedSystemFont(ofSize: 16, weight: .regular),
    .foregroundColor: NSColor.labelColor,
  ]

  override func draw(_ dirtyRect: NSRect) {
    super.draw(dirtyRect)
    NSColor.windowBackgroundColor.setFill()
    dirtyRect.fill()
    text.draw(at: NSPoint(x: 12, y: 12), withAttributes: attributes)
  }

  func truthBounds() -> NSRect {
    NSRect(origin: NSPoint(x: 12, y: 12), size: text.size(withAttributes: attributes))
  }
}

final class FixtureDelegate: NSObject, NSApplicationDelegate {
  private var window: NSWindow!
  private var popover: NSPopover?
  private var sameAppWindow: NSWindow?
  private var input: NSTextField!
  private var popoverButton: NSButton!
  private var sensitiveViews = [NSView]()
  private let burstLabel = NSTextField(labelWithString: "短周期表示: 待機中 / Rapid state: ready")
  private var burstTimer: Timer?
  private var burstIndex = 0
  private var inputBurstTimer: Timer?
  private var inputBurstIndex = 0
  private var timedInputTruth = [TimedFixtureRect]()
  private let inputBurstStates = [
    "山田 花子",
    "hanako@example.test",
    "確認コード 771244",
    "dsq_fixture_secret_7A91",
  ]

  func applicationDidFinishLaunching(_: Notification) {
    let stack = NSStackView()
    stack.orientation = .vertical
    stack.alignment = .leading
    stack.spacing = 18
    stack.edgeInsets = NSEdgeInsets(top: 32, left: 36, bottom: 32, right: 36)
    let slate = NSTextField(labelWithString: "DropSquash Secure Share Test / テスト専用フィクスチャ")
    slate.font = .systemFont(ofSize: 15, weight: .semibold)
    slate.textColor = .secondaryLabelColor
    stack.addArrangedSubview(slate)
    [
      "支払い失敗の再現調査 / Billing failure repro",
      "顧客名: 山田 花子 / Customer: Hanako Yamada",
      "メール: hanako@example.test / Code: 493-821",
      "電話: 080-0000-1234 / Session: 771244",
    ].forEach { value in
      let label = NSTextField(labelWithString: value)
      label.font = .systemFont(ofSize: 18)
      stack.addArrangedSubview(label)
      sensitiveViews.append(label)
    }
    input = NSTextField(string: "")
    input.placeholderString = "日本語入力の変換途中 / IME composition"
    input.widthAnchor.constraint(equalToConstant: 420).isActive = true
    stack.addArrangedSubview(input)
    let canvasSecret = CanvasSecretView(frame: NSRect(x: 0, y: 0, width: 420, height: 42))
    canvasSecret.widthAnchor.constraint(equalToConstant: 420).isActive = true
    canvasSecret.heightAnchor.constraint(equalToConstant: 42).isActive = true
    stack.addArrangedSubview(canvasSecret)
    sensitiveViews.append(canvasSecret)
    let button = NSButton(title: "確認ダイアログを表示 / Show confirmation", target: self, action: #selector(showConfirmation))
    stack.addArrangedSubview(button)
    popoverButton = NSButton(title: "一時ポップオーバーを表示 / Show transient popover", target: self, action: #selector(showPopover))
    stack.addArrangedSubview(popoverButton)
    let titleButton = NSButton(title: "ウィンドウ名を変更 / Change window title", target: self, action: #selector(changeTitle))
    stack.addArrangedSubview(titleButton)
    burstLabel.font = .monospacedSystemFont(ofSize: 14, weight: .regular)
    stack.addArrangedSubview(burstLabel)
    let burstButton = NSButton(title: "短周期の秘密表示を再現 / Run rapid secret burst", target: self, action: #selector(runRapidBurst))
    stack.addArrangedSubview(burstButton)
    let inputBurstButton = NSButton(title: "入力欄の短周期更新を再現 / Run rapid input updates", target: self, action: #selector(runRapidInputBurst))
    stack.addArrangedSubview(inputBurstButton)
    window = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 760, height: 620),
      styleMask: [.titled, .closable],
      backing: .buffered,
      defer: false
    )
    window.title = "DropSquash AX Fixture"
    window.contentView = stack
    window.center()
    window.makeKeyAndOrderFront(nil)
    NSApp.activate(ignoringOtherApps: true)
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) { [weak self] in
      self?.writeTruthIfRequested()
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_INPUT_BURST"] == "1" {
      let delay = fixtureDelay("DROP_SQUASH_FIXTURE_INPUT_BURST_DELAY_SECONDS")
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
        self?.runRapidInputBurst()
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_RAPID_BURST"] == "1" {
      let delay = fixtureDelay("DROP_SQUASH_FIXTURE_RAPID_BURST_DELAY_SECONDS")
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
        self?.runRapidBurst()
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_TITLE_CHANGE"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_TITLE_CHANGE_DELAY_SECONDS")) { [weak self] in
        self?.changeTitle()
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_POPOVER"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_POPOVER_DELAY_SECONDS")) { [weak self] in
        guard let self else { return }
        self.showPopover(self.popoverButton)
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_SHEET"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_SHEET_DELAY_SECONDS")) { [weak self] in
        self?.showConfirmation()
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_HIDE_WINDOW"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_HIDE_WINDOW_DELAY_SECONDS")) { [weak self] in
        self?.window.orderOut(nil)
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_RESIZE_WINDOW"] == "1" {
      let delay = fixtureDelay("DROP_SQUASH_FIXTURE_RESIZE_DELAY_SECONDS")
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
        guard let self else { return }
        var frame = self.window.frame
        frame.size.width += 80
        self.window.setFrame(frame, display: true)
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_MOVE_WINDOW"] == "1" {
      let delay = fixtureDelay("DROP_SQUASH_FIXTURE_MOVE_DELAY_SECONDS")
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
        guard let self else { return }
        var frame = self.window.frame
        frame.origin.x += 80
        self.window.setFrame(frame, display: true)
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_MOVE_RETURN"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_MOVE_RETURN_DELAY_SECONDS")) { [weak self] in
        guard let self else { return }
        let frame = self.window.frame
        var moved = frame
        moved.origin.x += 80
        self.window.setFrame(moved, display: true)
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
          self.window.setFrame(frame, display: true)
        }
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_FOREGROUND_SWITCH"] == "1" {
      let delay = fixtureDelay("DROP_SQUASH_FIXTURE_FOREGROUND_DELAY_SECONDS")
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
        NSWorkspace.shared.runningApplications.first {
          $0.bundleIdentifier == "com.apple.finder"
        }?.activate(options: [])
      }
    }
    if ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_AUTORUN_SAME_APP_WINDOW_FOCUS"] == "1" {
      DispatchQueue.main.asyncAfter(deadline: .now() + fixtureDelay("DROP_SQUASH_FIXTURE_SAME_APP_FOCUS_DELAY_SECONDS")) { [weak self] in
        self?.focusSameAppWindow()
      }
    }
  }

  private func fixtureDelay(_ key: String) -> Double {
    Double(ProcessInfo.processInfo.environment[key] ?? "10") ?? 10
  }

  private func writeTruthIfRequested() {
    guard let path = ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_TRUTH_PATH"] else { return }
    let sensitive = sensitiveViews.compactMap { view -> FixtureRect? in
      let bounds = (view as? CanvasSecretView)?.truthBounds() ?? view.bounds
      let rect = view.convert(bounds, to: nil)
      guard rect.width > 0 && rect.height > 0 else { return nil }
      return FixtureRect(
        x: Int(rect.minX.rounded()),
        y: Int((window.frame.height - rect.maxY).rounded()),
        width: Int(rect.width.rounded()),
        height: Int(rect.height.rounded())
      )
    }
    let truth = FixtureTruth(
      frameWidth: Int(window.frame.width.rounded()),
      frameHeight: Int(window.frame.height.rounded()),
      sensitive: sensitive,
      timedSensitive: timedInputTruth
    )
    guard let data = try? JSONEncoder().encode(truth) else { return }
    try? data.write(to: URL(fileURLWithPath: path), options: .atomic)
  }

  private func inputTruthRect(_ value: String) -> FixtureRect? {
    let font = input.font ?? NSFont.systemFont(ofSize: NSFont.systemFontSize)
    let size = value.size(withAttributes: [.font: font])
    let bounds = NSRect(x: 5, y: (input.bounds.height - size.height) / 2, width: size.width, height: size.height)
    let rect = input.convert(bounds, to: nil)
    guard rect.width > 0 && rect.height > 0 else { return nil }
    return FixtureRect(
      x: Int(rect.minX.rounded()),
      y: Int((window.frame.height - rect.maxY).rounded()),
      width: Int(rect.width.rounded()),
      height: Int(rect.height.rounded())
    )
  }

  @objc private func showConfirmation() {
    let alert = NSAlert()
    alert.messageText = "共有前の確認 / Confirm before sharing"
    alert.informativeText = "hanako@example.test の記録を削除しますか。確認コード: 771244"
    alert.addButton(withTitle: "キャンセル / Cancel")
    alert.beginSheetModal(for: window)
  }

  @objc private func showPopover(_ sender: NSButton) {
    let label = NSTextField(wrappingLabelWithString: "自動入力候補 / Autofill\nhanako@example.test\nOne-time code: 771244")
    label.frame = NSRect(x: 18, y: 18, width: 280, height: 74)
    let view = NSView(frame: NSRect(x: 0, y: 0, width: 316, height: 110))
    view.addSubview(label)
    let controller = NSViewController()
    controller.view = view
    let popover = NSPopover()
    popover.behavior = .transient
    popover.contentViewController = controller
    popover.show(relativeTo: sender.bounds, of: sender, preferredEdge: .maxY)
    self.popover = popover
  }

  @objc private func changeTitle() {
    window.title = window.title == "DropSquash AX Fixture"
      ? "Sensitive title state / 機密タイトル状態"
      : "DropSquash AX Fixture"
  }

  @objc private func runRapidBurst() {
    let states = [
      "候補: 山田 花子 / Candidate: Hanako Yamada",
      "確認コード: 771244 / One-time code: 771244",
      "Token: dsq_fixture_secret_7A91",
      "保存先: /Users/fixture/Private QA.mov",
    ]
    burstTimer?.invalidate()
    burstIndex = 0
    burstTimer = Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { [weak self] timer in
      guard let self else { timer.invalidate(); return }
      guard self.burstIndex < states.count * 3 else {
        self.burstLabel.stringValue = "短周期表示: 完了 / Rapid state: complete"
        timer.invalidate()
        return
      }
      self.burstLabel.stringValue = states[self.burstIndex % states.count]
      self.burstIndex += 1
    }
  }

  @objc private func runRapidInputBurst() {
    inputBurstTimer?.invalidate()
    inputBurstIndex = 0
    let cycles = Int(ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_INPUT_BURST_CYCLES"] ?? "3") ?? 3
    window.makeFirstResponder(input)
    inputBurstTimer = Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { [weak self] timer in
      guard let self else { timer.invalidate(); return }
      guard self.inputBurstIndex < self.inputBurstStates.count * cycles else {
        self.input.stringValue = ""
        self.closeTimedInputTruth()
        self.writeTruthIfRequested()
        timer.invalidate()
        return
      }
      self.input.stringValue = self.inputBurstStates[self.inputBurstIndex % self.inputBurstStates.count]
      self.recordTimedInputTruth(self.input.stringValue)
      self.inputBurstIndex += 1
    }
  }

  private func recordTimedInputTruth(_ value: String) {
    guard includesInputTruth() else { return }
    closeTimedInputTruth()
    guard let rect = inputTruthRect(value) else { return }
    timedInputTruth.append(TimedFixtureRect(startNs: monotonicNs(), endNs: UInt64.max, rect: rect))
    writeTruthIfRequested()
  }

  private func closeTimedInputTruth() {
    guard includesInputTruth() else { return }
    guard !timedInputTruth.isEmpty else { return }
    timedInputTruth[timedInputTruth.count - 1].endNs = monotonicNs()
  }

  private func monotonicNs() -> UInt64 {
    UInt64((ProcessInfo.processInfo.systemUptime * 1_000_000_000).rounded())
  }

  private func includesInputTruth() -> Bool {
    ProcessInfo.processInfo.environment["DROP_SQUASH_FIXTURE_TRUTH_INCLUDE_INPUT"] == "1"
  }

  private func focusSameAppWindow() {
    let field = NSTextField(string: "別ウィンドウの入力 / Other window input: dsq_same_app_771244")
    field.frame = NSRect(x: 24, y: 42, width: 420, height: 28)
    let content = NSView(frame: NSRect(x: 0, y: 0, width: 470, height: 112))
    content.addSubview(field)
    let window = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 470, height: 112),
      styleMask: [.titled, .closable],
      backing: .buffered,
      defer: false
    )
    window.title = "DropSquash AX Fixture secondary"
    window.contentView = content
    window.center()
    window.makeKeyAndOrderFront(nil)
    window.makeFirstResponder(field)
    sameAppWindow = window
  }
}

let app = NSApplication.shared
let delegate = FixtureDelegate()
app.delegate = delegate
app.setActivationPolicy(.regular)
app.run()
