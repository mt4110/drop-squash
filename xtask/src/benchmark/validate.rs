use dropsquash_core::EncodeResult;

pub fn result(result: &EncodeResult) -> Result<(), String> {
    if !result.is_successful_conversion() {
        return Err(format!(
            "benchmark output was not a successful smaller conversion: {}",
            result.output_path.display()
        ));
    }
    let metadata = std::fs::metadata(&result.output_path)
        .map_err(|error| format!("benchmark output is missing: {error}"))?;
    if metadata.len() == 0 {
        return Err(format!(
            "benchmark output is empty: {}",
            result.output_path.display()
        ));
    }
    if metadata.len() != result.output_bytes {
        return Err(format!(
            "benchmark output size changed before CSV evidence: {}",
            result.output_path.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
