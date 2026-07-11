use std::process::Command;

pub(super) fn macos_version() -> Result<String, String> {
    command_text("sw_vers", &["-productVersion"])
}

pub(super) fn machine_name() -> Result<String, String> {
    let arch = command_text("uname", &["-m"])?;
    let model = command_text("sysctl", &["-n", "hw.model"]).unwrap_or_default();
    if model.is_empty() {
        return Ok(arch);
    }
    Ok(format!("{model} {arch}"))
}

pub(super) fn tester_name() -> Result<String, String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .map_err(|_| "USER is not set".to_string())
}

pub(super) fn today() -> Result<String, String> {
    command_text("date", &["+%F"])
}

fn command_text(command: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!("{command} failed"));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| error.to_string())
        .map(|value| value.trim().to_string())
}
