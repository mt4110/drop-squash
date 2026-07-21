use std::process::Command;

mod bounds;

use bounds::parse;
const DEFAULT_OUTPUT: &str = "/tmp/dropsquash-window.png";
const DEFAULT_PROCESS: &str = "dropsquash-desktop";
const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-window-capture [output-path] [process-name]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (output, process) = parse_args(args)?;
    let output = resolved_output(&output);
    let bounds = capture_window(&output, &process)?;
    println!("window screenshot: {output}");
    println!("window bounds: {bounds}");
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, String), String> {
    match args.as_slice() {
        [] => Ok((DEFAULT_OUTPUT.into(), DEFAULT_PROCESS.into())),
        [output] => Ok((output.clone(), DEFAULT_PROCESS.into())),
        [output, process] => Ok((output.clone(), process.clone())),
        _ => Err(USAGE.into()),
    }
}

fn resolved_output(output: &str) -> String {
    if let Some(path) = output.strip_prefix("/tmp/") {
        return format!("/private/tmp/{path}");
    }
    output.to_string()
}

fn capture_window(output: &str, process: &str) -> Result<String, String> {
    let pid = process_pid(process)?;
    let info = Command::new("swift")
        .args(["-e", swift_window_info_script(), &pid])
        .output()
        .map_err(|error| format!("failed to capture window screenshot: {error}"))?;
    if !info.status.success() {
        return Err(String::from_utf8_lossy(&info.stderr).trim().to_string());
    }
    let text = String::from_utf8_lossy(&info.stdout).trim().to_string();
    let (owner, bounds) = text
        .split_once('|')
        .ok_or_else(|| format!("expected window owner and bounds, got {text}"))?;
    activate_owner(owner)?;
    Command::new("screencapture")
        .args(["-x", "-R", bounds, output])
        .status()
        .map_err(|error| format!("failed to run screencapture: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| "screencapture command failed".to_string())?;
    if !std::path::Path::new(output).exists() {
        return Err(format!("screenshot file was not created: {output}"));
    }
    parse(bounds)?;
    Ok(bounds.to_string())
}

fn process_pid(process: &str) -> Result<String, String> {
    let pgrep = Command::new("pgrep")
        .args(["-x", process])
        .output()
        .map_err(|error| format!("failed to read process id: {error}"))?;
    if pgrep.status.success() {
        let pid = String::from_utf8_lossy(&pgrep.stdout)
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .unwrap_or_default()
            .to_string();
        if !pid.is_empty() {
            return Ok(pid);
        }
    }
    let script = format!("tell application \"System Events\" to unix id of first application process whose name is \"{process}\"");
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|error| format!("failed to read process id: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    pid.parse::<i64>()
        .map_err(|error| format!("failed to parse process id: {error}"))?;
    Ok(pid)
}

fn swift_window_info_script() -> &'static str {
    r#"import CoreGraphics
import Foundation
let pid = Int32(CommandLine.arguments[1]) ?? 0
let list = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements], kCGNullWindowID) as? [[String: Any]] ?? []
for window in list {
  guard let ownerPid = window[kCGWindowOwnerPID as String] as? Int32, ownerPid == pid else { continue }
  guard let bounds = window[kCGWindowBounds as String] as? [String: Any],
        let owner = window[kCGWindowOwnerName as String] as? String,
        let x = bounds["X"] as? Double,
        let y = bounds["Y"] as? Double,
        let w = bounds["Width"] as? Double,
        let h = bounds["Height"] as? Double,
        w > 0, h > 0 else { continue }
  print("\(owner)|\(Int(x.rounded())),\(Int(y.rounded())),\(Int(w.rounded())),\(Int(h.rounded()))")
  exit(0)
}
fputs("window not found for pid \(pid)\n", stderr)
exit(1)"#
}
fn activate_owner(owner: &str) -> Result<(), String> {
    let script = format!("tell application \"{owner}\" to activate");
    Command::new("osascript")
        .args(["-e", &script])
        .status()
        .map_err(|error| format!("failed to activate window owner: {error}"))?
        .success()
        .then_some(())
        .ok_or_else(|| format!("activate command failed for {owner}"))
}

#[cfg(test)]
mod tests;
