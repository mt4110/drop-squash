mod parse;
mod validation;

use std::path::Path;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let path = match args.as_slice() {
        [path] => Path::new(path),
        _ => {
            return Err(
                "usage: cargo run -p xtask -- benchmark-csv-check path/to/results.csv".into(),
            );
        }
    };
    validate_path(path)
}

pub(crate) fn validate_path(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!("benchmark CSV does not exist: {}", path.display()));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read benchmark CSV {}: {error}", path.display()))?;
    validation::validate(&parse::parse(&text)?)
}

#[cfg(test)]
mod tests;
