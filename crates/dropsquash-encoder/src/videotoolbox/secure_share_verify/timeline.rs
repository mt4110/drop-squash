use dropsquash_core::{AppError, Result};
use objc2_core_media::{CMTime, CMTimeFlags};

const MAX_DECODED_FRAME_GAP_NS: u64 = 5 * (1_000_000_000 / 30);

#[derive(Default)]
pub(super) struct DecodedTimeline {
    last_ns: Option<u64>,
}

impl DecodedTimeline {
    pub(super) fn accept(&mut self, value: CMTime) -> Result<()> {
        let current = nanoseconds(value)?;
        if self.last_ns >= Some(current) {
            return Err(invalid(
                "decoded presentation time is not strictly increasing",
            ));
        }
        if self
            .last_ns
            .is_some_and(|last| current - last > MAX_DECODED_FRAME_GAP_NS)
        {
            return Err(invalid(
                "decoded presentation time gap exceeds the capture policy",
            ));
        }
        self.last_ns = Some(current);
        Ok(())
    }
}

fn nanoseconds(value: CMTime) -> Result<u64> {
    if !value.flags.contains(CMTimeFlags::Valid)
        || value.timescale <= 0
        || value.value < 0
        || value.epoch != 0
    {
        return Err(invalid("decoded presentation time is invalid"));
    }
    u64::try_from(
        u128::from(value.value as u64) * 1_000_000_000 / u128::from(value.timescale as u64),
    )
    .map_err(|_| invalid("decoded presentation time overflows nanoseconds"))
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share verification {reason}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(value: i64) -> CMTime {
        unsafe { CMTime::new(value, 600) }
    }

    #[test]
    fn accepts_decoded_frames_in_presentation_order() {
        let mut timeline = DecodedTimeline::default();
        assert!(timeline.accept(time(0)).is_ok());
        assert!(timeline.accept(time(20)).is_ok());
    }

    #[test]
    fn rejects_reordered_or_gapped_decoded_frames() {
        let mut timeline = DecodedTimeline::default();
        timeline.accept(time(20)).unwrap();
        assert!(timeline.accept(time(20)).is_err());
        let mut timeline = DecodedTimeline::default();
        timeline.accept(time(0)).unwrap();
        assert!(timeline.accept(time(101)).is_err());
    }
}
