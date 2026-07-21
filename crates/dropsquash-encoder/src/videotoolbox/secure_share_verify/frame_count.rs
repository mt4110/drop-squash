use dropsquash_core::{AppError, Result};

pub(super) fn verify(decoded: u64, planned: Option<usize>) -> Result<()> {
    if decoded == 0 {
        return Err(AppError::Encoder(
            "Secure Share verification found no output frames".to_string(),
        ));
    }
    if planned.is_some_and(|count| count as u64 != decoded) {
        return Err(AppError::InvalidConfig(
            "Secure Share verification frame count differs from MaskPlan".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::verify;

    #[test]
    fn rejects_no_decoded_frames() {
        assert!(verify(0, Some(1)).is_err());
    }

    #[test]
    fn rejects_a_plan_count_that_differs_from_decoded_output() {
        let error = verify(2, Some(1)).expect_err("frame mismatch must fail closed");
        assert!(error.to_string().contains("frame count differs"));
    }

    #[test]
    fn accepts_matching_frame_counts() {
        assert!(verify(2, Some(2)).is_ok());
    }
}
