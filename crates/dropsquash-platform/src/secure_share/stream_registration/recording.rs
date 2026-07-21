use std::path::PathBuf;

use dropsquash_core::{AxObservation, FrameSize, MaskPolicy, PixelRect, Result};
use objc2_screen_capture_kit::{SCContentFilter, SCStreamConfiguration};

use super::{
    SckFrameMetadataStreamRegistration, SckStreamCapturePlan, SckStreamFrameMetadataOutput,
};

impl SckFrameMetadataStreamRegistration {
    pub fn from_plan_recording(
        plan: &SckStreamCapturePlan,
        output_path: PathBuf,
        accessibility: Vec<AxObservation>,
        policy: MaskPolicy,
    ) -> Result<Self> {
        Self::new_recording(
            &plan.filter,
            &plan.configuration,
            plan.frame_size,
            output_path,
            accessibility,
            policy,
        )
    }

    pub fn new_recording(
        filter: &SCContentFilter,
        configuration: &SCStreamConfiguration,
        frame_size: FrameSize,
        output_path: PathBuf,
        accessibility: Vec<AxObservation>,
        policy: MaskPolicy,
    ) -> Result<Self> {
        Self::with_output(
            filter,
            configuration,
            SckStreamFrameMetadataOutput::new_recording(
                frame_size,
                output_path,
                fixed_mask_rects(frame_size, accessibility, policy),
                qa_vision_frame_limit(policy),
            ),
        )
    }

    pub fn finish_recording(&self) -> Result<Option<PathBuf>> {
        self.output.finish_recording()
    }

    pub fn discard_recording(&self) {
        self.output.discard_recording();
    }
}

fn qa_vision_frame_limit(policy: MaskPolicy) -> usize {
    if policy == MaskPolicy::SmartMask
        && std::env::var("DROP_SQUASH_QA_SMART_MASK_VISION_FIRST_FRAME_ONLY")
            .ok()
            .as_deref()
            == Some("1")
    {
        1
    } else {
        usize::MAX
    }
}

fn fixed_mask_rects(
    frame_size: FrameSize,
    accessibility: Vec<AxObservation>,
    policy: MaskPolicy,
) -> Vec<PixelRect> {
    let mut rects = Vec::new();
    if policy == MaskPolicy::StrictReveal {
        rects.push(PixelRect {
            x: 0,
            y: 0,
            width: frame_size.width,
            height: frame_size.height,
        });
    }
    rects.extend(accessibility.into_iter().map(|item| item.rect));
    rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_shield_starts_with_a_full_frame_mask() {
        let frame_size = FrameSize {
            width: 640,
            height: 480,
        };
        let rects = fixed_mask_rects(frame_size, Vec::new(), MaskPolicy::StrictReveal);

        assert_eq!(
            rects,
            vec![PixelRect {
                x: 0,
                y: 0,
                width: 640,
                height: 480,
            }]
        );
    }

    #[test]
    fn smart_mask_starts_with_only_accessibility_regions() {
        let rects = fixed_mask_rects(
            FrameSize {
                width: 640,
                height: 480,
            },
            Vec::new(),
            MaskPolicy::SmartMask,
        );

        assert!(rects.is_empty());
    }
}
