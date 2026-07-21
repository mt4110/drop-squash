use dropsquash_core::{FrameSize, MaskMode, PixelRect, SecureShareOptions};

use super::super::options_for_frame;
use super::plan;

#[test]
fn plan_options_reject_a_strict_plan_without_full_frame_destruction() {
    let options = SecureShareOptions {
        mask_mode: MaskMode::SolidBlack,
        mask_rects: Vec::new(),
        mask_plan: Some(partial_plan()),
    };
    let error = options_for_frame(
        &options,
        0,
        FrameSize {
            width: 4,
            height: 4,
        },
    )
    .expect_err("Strict Reveal must cover the entire decoded frame");

    assert!(error.to_string().contains("not fully covered"));
}

fn partial_plan() -> dropsquash_core::MaskPlan {
    let mut plan = plan();
    plan.frames[0].regions[0].rect = PixelRect {
        x: 1,
        y: 1,
        width: 2,
        height: 2,
    };
    plan
}
