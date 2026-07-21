pub(super) fn script(panel: &str, samples: &[String]) -> String {
    format!(
        r#"import ApplicationServices
import AppKit
import Foundation
let panel = "{panel}"
let files = [{files}].map {{ panel + "/" + $0 }}
func app(_ name: String) -> NSRunningApplication? {{ NSWorkspace.shared.runningApplications.first {{ $0.localizedName == name || $0.executableURL?.lastPathComponent == name }} }}
func get<T>(_ e: AXUIElement,_ a: String) -> T? {{ var v: CFTypeRef?; return AXUIElementCopyAttributeValue(e, a as CFString, &v) == .success ? (v as? T) : nil }}
func frame(_ e: AXUIElement) -> CGRect? {{ guard let p: AXValue = get(e, kAXPositionAttribute as String), let z: AXValue = get(e, kAXSizeAttribute as String) else {{ return nil }}; var point = CGPoint.zero; var size = CGSize.zero; AXValueGetValue(p, .cgPoint, &point); AXValueGetValue(z, .cgSize, &size); return CGRect(origin: point, size: size) }}
func dropTarget() -> CGPoint {{
  guard let pid = app("DropSquash")?.processIdentifier ?? app("dropsquash-desktop")?.processIdentifier else {{ return CGPoint(x: 960, y: 380) }}
  let root = AXUIElementCreateApplication(pid)
  let windows: [AXUIElement] = get(root, kAXWindowsAttribute as String) ?? []
  guard let rect = windows.compactMap(frame).first else {{ return CGPoint(x: 960, y: 380) }}
  return CGPoint(x: rect.midX, y: rect.midY)
}}
final class SourceView: NSView, NSDraggingSource {{
  let files: [URL]
  var started = false
  init(files: [String]) {{ self.files = files.map(URL.init(fileURLWithPath:)); super.init(frame: NSRect(x: 0, y: 0, width: 120, height: 120)) }}
  required init?(coder: NSCoder) {{ fatalError("init(coder:) has not been implemented") }}
  override func draw(_ dirtyRect: NSRect) {{ NSColor.windowBackgroundColor.setFill(); dirtyRect.fill(); for (index, file) in files.enumerated() {{ NSWorkspace.shared.icon(forFile: file.path).draw(in: NSRect(x: 20 + (index * 18), y: 28, width: 48, height: 48)) }} }}
  override func mouseDown(with event: NSEvent) {{
    guard !started else {{ return }}
    started = true
    print("mouseDown starting drag for \(files.map(\.lastPathComponent))")
    if files.count == 1 {{
      _ = dragFile(files[0].path, from: NSRect(x: 20, y: 28, width: 48, height: 48), slideBack: true, event: event)
      return
    }}
    let items = files.enumerated().map {{ index, file in
      let item = NSDraggingItem(pasteboardWriter: file as NSURL)
      item.setDraggingFrame(NSRect(x: 20 + (index * 18), y: 28, width: 48, height: 48), contents: NSWorkspace.shared.icon(forFile: file.path))
      return item
    }}
    let session = beginDraggingSession(with: items, event: event, source: self)
    session.animatesToStartingPositionsOnCancelOrFail = true
  }}
  func draggingSession(_ session: NSDraggingSession, sourceOperationMaskFor context: NSDraggingContext) -> NSDragOperation {{ .copy }}
  func ignoreModifierKeys(for session: NSDraggingSession) -> Bool {{ true }}
  func draggedImage(_ image: NSImage, endedAt point: NSPoint, operation: NSDragOperation) {{ print("draggedImage endedAt operation=\(operation.rawValue) point=\(point)"); DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {{ NSApp.terminate(nil) }} }}
}}
func post(_ type: CGEventType, _ point: CGPoint) {{ let src = CGEventSource(stateID: .hidSystemState); CGEvent(mouseEventSource: src, mouseType: type, mouseCursorPosition: point, mouseButton: .left)!.post(tap: .cghidEventTap) }}
func postMouseDown(_ window: NSWindow) {{
  guard let event = NSEvent.mouseEvent(with: .leftMouseDown, location: CGPoint(x: 60, y: 60), modifierFlags: [], timestamp: 0, windowNumber: window.windowNumber, context: nil, eventNumber: 1, clickCount: 1, pressure: 1) else {{
    fputs("failed to post mouseDown event\n", stderr)
    exit(2)
  }}
  NSApp.postEvent(event, atStart: false)
}}
let target = dropTarget()
let origin = CGPoint(x: 180, y: 180)
let app = NSApplication.shared
app.setActivationPolicy(.regular)
let window = NSWindow(contentRect: NSRect(x: origin.x - 60, y: origin.y - 60, width: 120, height: 120), styleMask: [.borderless], backing: .buffered, defer: false)
window.level = .floating
window.isOpaque = false
window.backgroundColor = .clear
window.contentView = SourceView(files: files)
window.makeKeyAndOrderFront(nil)
window.orderFrontRegardless()
app.activate(ignoringOtherApps: true)
DispatchQueue.global().asyncAfter(deadline: .now() + 8) {{ fputs("manual QA drag helper timed out\n", stderr); exit(2) }}
DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {{
  print("drag helper origin=\(origin) target=\(target)")
  CGWarpMouseCursorPosition(origin)
  postMouseDown(window)
  DispatchQueue.global().asyncAfter(deadline: .now() + 0.1) {{
    post(.leftMouseDown, origin)
    usleep(60_000)
    for step in 1...28 {{
      let t = CGFloat(step) / 28
      let point = CGPoint(x: origin.x + ((target.x - origin.x) * t), y: origin.y + ((target.y - origin.y) * t))
      post(.leftMouseDragged, point)
      usleep(18_000)
    }}
    post(.leftMouseUp, target)
  }}
}}
app.run()"#,
        panel = panel.replace('\\', "\\\\").replace('"', "\\\""),
        files = samples
            .iter()
            .map(|sample| format!("\"{}\"", sample.replace('\\', "\\\\").replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::script;

    #[test]
    fn script_uses_native_drag_session_for_file_urls() {
        let script = script(
            "/private/tmp/dropsquash-qa-open-panel",
            &["qa-small.mov".into(), "qa-medium.mov".into()],
        );
        assert!(script.contains(
            "let files = [\"qa-small.mov\", \"qa-medium.mov\"].map { panel + \"/\" + $0 }"
        ));
        assert!(script.contains("dragFile(files[0].path"));
        assert!(script.contains("beginDraggingSession(with: items, event: event, source: self)"));
        assert!(script.contains("draggingSession(_ session: NSDraggingSession, sourceOperationMaskFor context: NSDraggingContext)"));
        assert!(script.contains("draggedImage endedAt operation="));
        assert!(script.contains("NSApp.postEvent(event, atStart: false)"));
        assert!(script.contains("manual QA drag helper timed out"));
        assert!(script.contains("mouseDown starting drag"));
        assert!(script.contains("post(.leftMouseDown, origin)"));
        assert!(!script.contains("view.mouseDown(with: event)"));
    }
}
