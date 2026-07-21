pub(super) fn reject_unmasked_text(count: u64) -> Result<(), String> {
    if count == 0 {
        return Ok(());
    }
    Err(format!(
        "Experimental QA output rejected: decoded verification found {count} unmasked text region(s)"
    ))
}

#[cfg(test)]
mod tests {
    use super::reject_unmasked_text;

    #[test]
    fn rejects_one_or_more_decoded_text_residuals() {
        assert!(reject_unmasked_text(0).is_ok());
        let error = reject_unmasked_text(1).unwrap_err();
        assert!(error.contains("1 unmasked text region"));
    }
}
