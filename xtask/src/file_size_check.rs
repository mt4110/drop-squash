use std::path::{Path, PathBuf};

const RUST_PRODUCTION_LIMIT: usize = 128;
const TYPESCRIPT_LIMIT: usize = 512;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let root = match args.as_slice() {
        [] => PathBuf::from("."),
        [root] => PathBuf::from(root),
        _ => return Err("file-size-check accepts at most one <root>".to_string()),
    };
    let violations = check_root(&root)?;
    if violations.is_empty() {
        println!("file size checks passed");
        return Ok(());
    }
    Err(violations.join("\n"))
}

fn check_root(root: &Path) -> Result<Vec<String>, String> {
    let mut violations = Vec::new();
    visit(root, &mut violations)?;
    Ok(violations)
}

fn visit(path: &Path, violations: &mut Vec<String>) -> Result<(), String> {
    if should_skip(path) {
        return Ok(());
    }
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|error| error.to_string())? {
            visit(
                &entry.map_err(|error| error.to_string())?.path(),
                violations,
            )?;
        }
        return Ok(());
    }
    check_file(path, violations)
}

fn check_file(path: &Path, violations: &mut Vec<String>) -> Result<(), String> {
    let Some(limit) = limit_for(path) else {
        return Ok(());
    };
    let count = line_count(path)?;
    if count > limit {
        violations.push(format!(
            "{} has {count} lines; limit is {limit}",
            path.display()
        ));
    }
    Ok(())
}

fn limit_for(path: &Path) -> Option<usize> {
    let extension = path.extension().and_then(|value| value.to_str())?;
    match extension {
        "rs" if is_rust_test(path) => None,
        "rs" => Some(RUST_PRODUCTION_LIMIT),
        "ts" | "tsx" => Some(TYPESCRIPT_LIMIT),
        _ => None,
    }
}

fn is_rust_test(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| name == "tests.rs")
        || path
            .components()
            .any(|part| part.as_os_str().to_string_lossy() == "tests")
}

fn line_count(path: &Path) -> Result<usize, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(text.lines().count())
}

fn should_skip(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| matches!(name, ".git" | ".private_docs" | "target" | "node_modules"))
}

#[cfg(test)]
mod tests;
