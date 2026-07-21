use serde_json::Value;

pub(super) fn path_set(values: &[Value], required: &[&str], label: &str) -> Result<(), String> {
    if values.len() != required.len() {
        return Err(format!(
            "{label} must contain exactly {} paths",
            required.len()
        ));
    }
    for value in values {
        let Some(path) = value.as_str() else {
            return Err(format!("{label} contains a non-string path"));
        };
        if !required.contains(&path) {
            return Err(format!("{label} contains an unknown path: {path}"));
        }
    }
    Ok(())
}

pub(super) fn classification_set(rows: &[Value], required: &[&str]) -> Result<(), String> {
    if rows.len() != required.len() {
        return Err(format!(
            "exposureClassification must contain exactly {} rows",
            required.len()
        ));
    }
    for row in rows {
        let Some(path) = row.get("path").and_then(Value::as_str) else {
            return Err("exposureClassification contains a row without path".to_string());
        };
        if !required.contains(&path) {
            return Err(format!(
                "exposureClassification contains unknown path: {path}"
            ));
        }
    }
    Ok(())
}
