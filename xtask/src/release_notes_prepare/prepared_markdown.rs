use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(super) fn write(path: &Path, lines: &[String]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("failed to create release notes Markdown output: {error}"))?;
    writeln!(file, "{}", lines.join("\n"))
        .map_err(|error| format!("failed to write release notes Markdown output: {error}"))?;
    println!("release notes Markdown output: {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests;
