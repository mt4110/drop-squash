use std::process::Command;

const DEFAULT_BINARY: &str = "/Volumes/DropSquash/DropSquash.app/Contents/MacOS/dropsquash-desktop";
const DEFAULT_PROCESS: &str = "DropSquash";
const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-window-probe [mounted-binary-path] [process-name]";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (binary, process) = parse_args(args)?;
    let label = probe_label(&binary);
    let pids = pid_list(&binary, &process)?;
    println!("{label} pid count: {}", pids.len());
    println!(
        "{label} pid list: {}",
        if pids.is_empty() {
            "<none>".into()
        } else {
            pids.join(" ")
        }
    );
    match window_count(&process) {
        Ok(count) => println!("{label} ax window count: {count}"),
        Err(error) => {
            println!("{label} ax window count: unavailable");
            println!("{label} ax note: {error}");
        }
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(String, String), String> {
    match args.as_slice() {
        [] => Ok((DEFAULT_BINARY.into(), DEFAULT_PROCESS.into())),
        [binary] => Ok((binary.clone(), DEFAULT_PROCESS.into())),
        [binary, process] => Ok((binary.clone(), process.clone())),
        _ => Err(USAGE.into()),
    }
}

fn pid_list(binary: &str, process: &str) -> Result<Vec<String>, String> {
    let output = Command::new("pgrep")
        .args(["-f", binary])
        .output()
        .map_err(|error| format!("failed to run pgrep: {error}"))?;
    let pids = parse_pids(output.status.success(), &output.stdout);
    if !pids.is_empty() {
        return Ok(pids);
    }
    ax_pid_list(process)
}

fn parse_pids(success: bool, stdout: &[u8]) -> Vec<String> {
    if !success {
        return Vec::new();
    }
    String::from_utf8_lossy(stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn ax_pid_list(process: &str) -> Result<Vec<String>, String> {
    let script = format!(
        "tell application \"System Events\" to unix id of every application process whose name is \"{process}\""
    );
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|error| format!("failed to run osascript for pid list: {error}"))?;
    if !output.status.success() {
        return Ok(Vec::new());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect())
}

fn probe_label(binary: &str) -> &'static str {
    if binary.starts_with("/Volumes/") {
        return "mounted dmg";
    }
    "app"
}

fn window_count(process: &str) -> Result<u32, String> {
    let script = format!(
        "tell application \"System Events\" to count windows of application process \"{process}\""
    );
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|error| format!("failed to run osascript: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    text.trim()
        .parse::<u32>()
        .map_err(|error| format!("failed to parse AX window count: {error}"))
}

#[cfg(test)]
mod tests;
