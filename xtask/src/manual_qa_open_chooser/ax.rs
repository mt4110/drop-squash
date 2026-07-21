pub(super) fn script(panel: Option<&str>, sample: Option<&str>) -> Option<String> {
    panel.map(|panel| {
        format!(
            r#"import ApplicationServices
import Cocoa
import Foundation
let panel = "{panel}"
let sample: String? = {sample}
let panelName = "{panel_name}"
let targetPath = panel
guard let appPid = NSWorkspace.shared.runningApplications.first(where: {{ $0.localizedName == "DropSquash" || $0.executableURL?.lastPathComponent == "dropsquash-desktop" }})?.processIdentifier else {{ fputs("failed to find DropSquash process\n", stderr); exit(1) }}
let app = AXUIElementCreateApplication(appPid)
func get<T>(_ e: AXUIElement,_ a: String) -> T? {{ var v: CFTypeRef?; return AXUIElementCopyAttributeValue(e, a as CFString, &v) == .success ? (v as? T) : nil }}
func s(_ e: AXUIElement,_ a: String) -> String {{ get(e, a) ?? "" }}
func kids(_ e: AXUIElement) -> [AXUIElement] {{ get(e, kAXChildrenAttribute as String) ?? [] }}
func click(_ p: CGPoint, _ count: Int = 1) {{ let src = CGEventSource(stateID: .hidSystemState); for type in [CGEventType.leftMouseDown, .leftMouseUp] {{ let event = CGEvent(mouseEventSource: src, mouseType: type, mouseCursorPosition: p, mouseButton: .left)!; event.setIntegerValueField(.mouseEventClickState, value: Int64(count)); event.post(tap: .cghidEventTap) }} }}
func key(_ code: CGKeyCode, _ flags: CGEventFlags = []) {{ let src = CGEventSource(stateID: .hidSystemState); let down = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: true)!; down.flags = flags; down.post(tap: .cghidEventTap); let up = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: false)!; up.flags = flags; up.post(tap: .cghidEventTap) }}
func fields(in e: AXUIElement) -> [AXUIElement] {{ (s(e, kAXRoleAttribute as String) == "AXTextField" ? [e] : []) + kids(e).flatMap(fields) }}
func button(titled target: String, in e: AXUIElement) -> AXUIElement? {{ if s(e, kAXRoleAttribute as String) == "AXButton" && s(e, kAXTitleAttribute as String) == target {{ return e }}; for child in kids(e) {{ if let found = button(titled: target, in: child) {{ return found }} }}; return nil }}
func press(_ e: AXUIElement) -> Bool {{ AXUIElementPerformAction(e, kAXPressAction as CFString) == .success }}
func actions(_ e: AXUIElement) -> [String] {{ var names: CFArray?; return AXUIElementCopyActionNames(e, &names) == .success ? (names as? [String] ?? []) : [] }}
func waitForInnerSheet() -> AXUIElement? {{ for _ in 0..<30 {{ if let ws: [AXUIElement] = get(app, kAXWindowsAttribute as String), let win = ws.first, let outer = kids(win).first(where: {{ s($0, kAXRoleAttribute as String) == "AXSheet" }}), let inner = kids(outer).first(where: {{ s($0, kAXRoleAttribute as String) == "AXSheet" }}) {{ return inner }}; usleep(100_000) }}; return nil }}
func outerSheet() -> AXUIElement? {{ let ws: [AXUIElement]? = get(app, kAXWindowsAttribute as String); return ws?.first.flatMap {{ kids($0).first(where: {{ s($0, kAXRoleAttribute as String) == "AXSheet" }}) }} }}
func frame(_ e: AXUIElement) -> CGRect? {{ guard let pos: AXValue = get(e, kAXPositionAttribute as String), let size: AXValue = get(e, kAXSizeAttribute as String) else {{ return nil }}; var p = CGPoint.zero; var z = CGSize.zero; AXValueGetValue(pos, .cgPoint, &p); AXValueGetValue(size, .cgSize, &z); return CGRect(origin: p, size: z) }}
func row(named target: String, in e: AXUIElement) -> AXUIElement? {{ if s(e, kAXRoleAttribute as String) == "AXGroup" && kids(e).contains(where: {{ s($0, kAXRoleAttribute as String) == "AXTextField" && s($0, kAXValueAttribute as String) == target }}) {{ return e }}; for child in kids(e) {{ if let found = row(named: target, in: child) {{ return found }} }}; return nil }}
func lists(in e: AXUIElement) -> [AXUIElement] {{ (s(e, kAXRoleAttribute as String) == "AXList" ? [e] : []) + kids(e).flatMap(lists) }}
func sampleSteps(_ sample: String) -> Int {{ switch sample {{ case "qa-invalid.mp4": return 1; case "qa-large.mov", "qa-large.mp4": return 2; case "qa-medium.mov": return 3; case "qa-not-smaller.mp4": return 4; case "qa-small.mov": return 5; default: return 1 }} }}
NSWorkspace.shared.runningApplications.first(where: {{ $0.processIdentifier == appPid }})?.activate(options: [.activateIgnoringOtherApps]); usleep(300_000)
if outerSheet() == nil, let ws: [AXUIElement] = get(app, kAXWindowsAttribute as String), let win = ws.first, let choose = button(titled: "Choose recording", in: win), let hit = frame(choose)?.center {{ click(hit); usleep(600_000) }}
if outerSheet() == nil, let ws: [AXUIElement] = get(app, kAXWindowsAttribute as String), let win = ws.first, let choose = button(titled: "Choose recording", in: win) {{ _ = press(choose); usleep(600_000) }}
key(5, [.maskCommand, .maskShift]); usleep(400_000)
guard let inner = waitForInnerSheet(), let field = kids(inner).first(where: {{ s($0, kAXRoleAttribute as String) == "AXTextField" }}) else {{ fputs("failed to open Go to Folder\\n", stderr); exit(1) }}
guard AXUIElementSetAttributeValue(field, kAXValueAttribute as CFString, targetPath as CFTypeRef) == .success else {{ fputs("failed to set folder path\\n", stderr); exit(1) }}
key(36); usleep(700_000)
if let outer = outerSheet(), let folder = row(named: panelName, in: outer), let hit = frame(folder)?.center {{ click(hit); usleep(140_000); if sample != nil {{ click(hit, 2); usleep(260_000) }} }}
if let outer = outerSheet(), let sample, let list = lists(in: outer).last, let row = row(named: sample, in: list), let field = kids(row).first(where: {{ s($0, kAXRoleAttribute as String) == "AXTextField" && s($0, kAXValueAttribute as String) == sample }}) {{
  if actions(field).contains("AXOpen") && AXUIElementPerformAction(field, "AXOpen" as CFString) == .success {{ usleep(260_000) }}
  else if actions(field).contains("AXConfirm") && AXUIElementPerformAction(field, "AXConfirm" as CFString) == .success {{ usleep(260_000) }}
  else if let hit = frame(row)?.center {{ click(hit); usleep(140_000); click(hit, 2); usleep(220_000) }}
}}
if let outer = outerSheet(), let sample, let search = fields(in: outer).first(where: {{ s($0, kAXValueAttribute as String).isEmpty }}), let split = kids(outer).first(where: {{ s($0, kAXRoleAttribute as String) == "AXSplitGroup" }}), let open = kids(split).first(where: {{ s($0, kAXRoleAttribute as String) == "AXButton" && s($0, kAXTitleAttribute as String) == "Open" }}), !(((get(open, kAXEnabledAttribute as String) as Bool?) ?? false)) {{ _ = AXUIElementSetAttributeValue(search, kAXValueAttribute as CFString, sample as CFTypeRef); usleep(260_000); key(125); usleep(120_000); for _ in 1..<sampleSteps(sample) {{ key(125); usleep(90_000) }}; key(36); usleep(240_000) }}
if let outer = outerSheet(), let open = kids(kids(outer).first(where: {{ s($0, kAXRoleAttribute as String) == "AXSplitGroup" }})!).first(where: {{ s($0, kAXRoleAttribute as String) == "AXButton" && s($0, kAXTitleAttribute as String) == "Open" }}), ((get(open, kAXEnabledAttribute as String) as Bool?) ?? false), let hit = frame(open)?.center {{ click(hit) }}
extension CGRect {{ var center: CGPoint {{ CGPoint(x: midX, y: midY) }} }}"#,
            panel = panel.replace('\\', "\\\\").replace('"', "\\\""),
            panel_name = panel
                .rsplit('/')
                .next()
                .unwrap_or(panel)
                .replace('\\', "\\\\")
                .replace('"', "\\\""),
            sample = sample
                .map(|value| format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")))
                .unwrap_or_else(|| "nil".to_string())
        )
    })
}

#[cfg(test)]
mod tests {
    use super::script;

    #[test]
    fn script_uses_folder_navigation_and_optional_sample_click() {
        let script = script(
            Some("/private/tmp/dropsquash-qa-open-panel"),
            Some("qa-small.mov"),
        )
        .unwrap();
        assert!(script.contains("key(5, [.maskCommand, .maskShift])"));
        assert!(script.contains("targetPath"));
        assert!(script.contains("AXUIElementSetAttributeValue"));
        assert!(script.contains("fields(in e: AXUIElement)"));
        assert!(script.contains("func button(titled target: String"));
        assert!(script.contains("func press(_ e: AXUIElement)"));
        assert!(script.contains("func actions(_ e: AXUIElement)"));
        assert!(script.contains("lists(in e: AXUIElement)"));
        assert!(script.contains("sampleSteps"));
        assert!(script.contains("button(titled: \"Choose recording\", in: win)"));
        assert!(script.contains("press(choose)"));
        assert!(script.contains("row(named: sample"));
        assert!(script.contains("AXUIElementPerformAction(field, \"AXOpen\" as CFString)"));
        assert!(script.contains("AXUIElementPerformAction(field, \"AXConfirm\" as CFString)"));
        assert!(script.contains("kAXValueAttribute as CFString, sample as CFTypeRef"));
        assert!(script.contains("lists(in: outer).last"));
        assert!(script.contains("panelName"));
        assert!(script.contains("Open"));
        assert!(script.contains("/private/tmp/dropsquash-qa-open-panel"));
        assert!(script.contains("\"qa-small.mov\""));
    }
}
