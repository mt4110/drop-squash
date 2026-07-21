use dropsquash_core::{AppError, Result};

pub(super) fn verify(track_count: usize, has_nonvisual_track: bool) -> Result<()> {
    if track_count != 1 || has_nonvisual_track {
        return Err(AppError::Encoder(
            "Secure Share verification found a non-video output track".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::verify;

    #[test]
    fn accepts_one_visual_video_track() {
        verify(1, false).unwrap();
    }

    #[test]
    fn rejects_missing_extra_or_nonvisual_tracks() {
        for (count, nonvisual) in [(0, false), (2, false), (1, true)] {
            let error = verify(count, nonvisual).unwrap_err().to_string();
            assert!(error.contains("non-video output track"));
        }
    }
}
