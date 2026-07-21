use dropsquash_core::{CaptureFrameMetadata, FrameStatus};

const MAX_CAPTURE_FRAME_GAP_NS: u64 = 5 * (1_000_000_000 / 30);

pub(super) fn validate(frames: &[CaptureFrameMetadata]) -> Result<(), String> {
    let Some(first) = frames.first() else {
        return Err("Secure Share native recording produced no frame metadata".to_string());
    };
    for (index, frame) in frames.iter().enumerate() {
        if frame.frame_index != index as u64 || frame.frame_status != FrameStatus::Complete {
            return Err(
                "Secure Share native frame metadata is not a complete sequence".to_string(),
            );
        }
        if index > 0 && invalid_transition(&frames[index - 1], frame, first) {
            return Err("Secure Share native frame continuity could not be verified".to_string());
        }
    }
    Ok(())
}

fn invalid_transition(
    previous: &CaptureFrameMetadata,
    current: &CaptureFrameMetadata,
    first: &CaptureFrameMetadata,
) -> bool {
    current.presentation_time_ns <= previous.presentation_time_ns
        || current.presentation_time_ns - previous.presentation_time_ns > MAX_CAPTURE_FRAME_GAP_NS
        || current.content_rect != first.content_rect
        || current.bounding_rect != first.bounding_rect
        || current.scale_factor.to_bits() != first.scale_factor.to_bits()
        || current.content_scale.to_bits() != first.content_scale.to_bits()
}

#[cfg(test)]
mod tests {
    use dropsquash_core::{CaptureRect, FrameStatus};

    use super::*;

    fn frame(index: u64, time: u64) -> CaptureFrameMetadata {
        CaptureFrameMetadata {
            frame_index: index,
            presentation_time_ns: time,
            frame_status: FrameStatus::Complete,
            content_rect: CaptureRect {
                x: 0,
                y: 0,
                width: 4,
                height: 4,
            },
            bounding_rect: CaptureRect {
                x: 0,
                y: 0,
                width: 4,
                height: 4,
            },
            scale_factor: 1.0,
            content_scale: 1.0,
        }
    }

    #[test]
    fn accepts_a_complete_continuous_native_sequence() {
        assert!(validate(&[frame(0, 1), frame(1, 33_000_000)]).is_ok());
    }

    #[test]
    fn rejects_a_gap_or_duplicate_frame_time() {
        assert!(validate(&[frame(0, 1), frame(1, 1)]).is_err());
        assert!(validate(&[frame(0, 1), frame(1, 200_000_000)]).is_err());
    }
}
