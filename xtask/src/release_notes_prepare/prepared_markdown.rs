use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub(super) fn write(path: &Path, lines: &[String]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("failed to create release notes Markdown output: {error}"))?;
    writeln!(file, "{}", markdown_text(lines))
        .map_err(|error| format!("failed to write release notes Markdown output: {error}"))?;
    println!("release notes Markdown output: {}", path.display());
    Ok(())
}

fn markdown_text(lines: &[String]) -> String {
    format!(
        "{}\n\n{}",
        "Prepared draft only. Replace every pending line before public release.",
        lines.join("\n")
    )
}

#[cfg(test)]
mod tests;
