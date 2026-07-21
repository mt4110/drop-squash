mod ax;

const USAGE: &str = "usage: cargo run -p xtask -- manual-qa-open-chooser [panel-dir] [sample-name]";

#[cfg(test)]
mod tests;

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (panel, sample) = parse_args(args)?;
    if panel.is_some() {
        run_swift_helper(panel.as_deref(), sample.as_deref())?;
        println!("manual QA chooser shortcut sent");
        return Ok(());
    }
    let status = std::process::Command::new("osascript")
        .args(script_args(panel.as_deref(), sample.as_deref()))
        .status()
        .map_err(|error| format!("failed to run osascript: {error}"))?;
    if !status.success() {
        return Err(format!("osascript exited with status {status}"));
    }
    println!("manual QA chooser shortcut sent");
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<(Option<String>, Option<String>), String> {
    match args.as_slice() {
        [] => Ok((None, None)),
        [panel] => Ok((Some(panel.clone()), None)),
        [panel, sample] => Ok((Some(panel.clone()), Some(sample.clone()))),
        _ => Err(USAGE.to_string()),
    }
}

fn script_args(panel: Option<&str>, sample: Option<&str>) -> Vec<String> {
    vec![
        "-e".to_string(),
        "tell application \"DropSquash\" to activate".to_string(),
        "-e".to_string(),
        "tell application \"System Events\" to set frontmost of process \"dropsquash-desktop\" to true".to_string(),
        "-e".to_string(),
        "delay 0.1".to_string(),
        "-e".to_string(),
        chooser_script(panel, sample).to_string(),
    ]
}

fn run_swift_helper(panel: Option<&str>, sample: Option<&str>) -> Result<(), String> {
    let Some(script) = ax::script(panel.map(normalize_tmp_path).as_deref(), sample) else {
        return Ok(());
    };
    let mut child = std::process::Command::new("swift")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to run swift helper: {error}"))?;
    use std::io::Write;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| "swift helper stdin unavailable".to_string())?
        .write_all(script.as_bytes())
        .map_err(|error| format!("failed to write swift helper: {error}"))?;
    let status = child
        .wait()
        .map_err(|error| format!("failed to wait for swift helper: {error}"))?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| format!("swift helper exited with status {status}"))
}

fn chooser_script(panel: Option<&str>, sample: Option<&str>) -> String {
    let mut script = "tell application \"System Events\"\nset processName to \"DropSquash\"\nif not (exists process processName) then\nset processName to \"dropsquash-desktop\"\nend if\nset openedChooser to false\nif exists button \"OK\" of window 1 of process processName then\nclick button \"OK\" of window 1 of process processName\ndelay 0.15\nend if\nif exists button \"Choose recording\" of window 1 of process processName then\nclick button \"Choose recording\" of window 1 of process processName\ndelay 0.2\nend if\nif (count of sheets of window 1 of process processName) > 0 then\nset sheetTexts to name of every static text of splitter group 1 of sheet 1 of window 1 of process processName\nif sheetTexts contains \"Choose a recording\" then\nset openedChooser to true\nend if\nend if\nif openedChooser is false then\nrepeat 6 times\nkey code 49\ndelay 0.15\nif (count of sheets of window 1 of process processName) > 0 then\nset sheetTexts to name of every static text of splitter group 1 of sheet 1 of window 1 of process processName\nif sheetTexts contains \"Choose a recording\" then\nset openedChooser to true\nexit repeat\nend if\nkey code 53\ndelay 0.1\nkey code 48 using shift down\ndelay 0.05\nelse\nkey code 48 using shift down\ndelay 0.05\nend if\nend repeat\nend if\nif openedChooser is false then\nerror \"failed to open recording chooser\"\nend if".to_string();
    let _ = (panel, sample);
    script.push_str("\nend tell");
    script
}

fn normalize_tmp_path(path: &str) -> String {
    path.strip_prefix("/tmp/")
        .map(|suffix| format!("/private/tmp/{suffix}"))
        .unwrap_or_else(|| path.to_string())
}
