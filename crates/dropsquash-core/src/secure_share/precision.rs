use super::{coverage, export_options::effective_rects, FrameSize, MaskPlan, PixelRect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectiveMaskPrecision {
    pub min_truth_covered_ppm: u32,
    pub max_mask_outside_truth_ppm: u32,
}

pub fn selective_mask_precision(plan: &MaskPlan, truth: &[PixelRect]) -> SelectiveMaskPrecision {
    selective_mask_precision_for_frames(plan, &vec![truth.to_vec(); plan.frames.len()])
        .expect("one truth set per frame")
}

pub fn selective_mask_precision_for_frames(
    plan: &MaskPlan,
    truth_by_frame: &[Vec<PixelRect>],
) -> Option<SelectiveMaskPrecision> {
    (plan.frames.len() == truth_by_frame.len()).then_some(())?;
    let mut minimum = 1_000_000;
    let mut maximum = 0;
    for (frame, truth) in plan.frames.iter().zip(truth_by_frame) {
        let truth_area = coverage::union_area(truth, plan.frame_size);
        if truth_area == 0 {
            return None;
        }
        let mask = effective_rects(frame, plan.frame_size);
        let covered = intersection_area(&mask, truth, plan.frame_size);
        let mask_area = coverage::union_area(&mask, plan.frame_size);
        minimum = minimum.min(coverage::ppm(covered, truth_area));
        maximum = maximum.max(coverage::ppm(
            mask_area.saturating_sub(covered),
            coverage::frame_area(plan.frame_size),
        ));
    }
    Some(SelectiveMaskPrecision {
        min_truth_covered_ppm: minimum,
        max_mask_outside_truth_ppm: maximum,
    })
}

fn intersection_area(left: &[PixelRect], right: &[PixelRect], size: FrameSize) -> u64 {
    let intersections = left
        .iter()
        .flat_map(|left| {
            right
                .iter()
                .filter_map(move |right| intersection(*left, *right))
        })
        .collect::<Vec<_>>();
    coverage::union_area(&intersections, size)
}

#[cfg(test)]
#[path = "precision/tests.rs"]
mod tests;

fn intersection(left: PixelRect, right: PixelRect) -> Option<PixelRect> {
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    let edge_x = left
        .x
        .saturating_add(left.width)
        .min(right.x.saturating_add(right.width));
    let edge_y = left
        .y
        .saturating_add(left.height)
        .min(right.y.saturating_add(right.height));
    (edge_x > x && edge_y > y).then_some(PixelRect {
        x,
        y,
        width: edge_x - x,
        height: edge_y - y,
    })
}
