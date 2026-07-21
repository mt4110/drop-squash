use super::{FrameSize, MaskPlan, PixelRect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DestructiveCoverage {
    pub max_frame_ppm: u32,
    pub mean_frame_ppm: u32,
}

pub fn destructive_coverage(plan: &MaskPlan) -> DestructiveCoverage {
    let coverages = plan
        .frames
        .iter()
        .map(|frame| {
            coverage_ppm(
                &frame
                    .regions
                    .iter()
                    .map(|region| region.rect)
                    .collect::<Vec<_>>(),
                plan.frame_size,
            )
        })
        .collect::<Vec<_>>();
    let sum = coverages.iter().map(|value| u64::from(*value)).sum::<u64>();
    DestructiveCoverage {
        max_frame_ppm: coverages.iter().copied().max().unwrap_or_default(),
        mean_frame_ppm: (sum / coverages.len().max(1) as u64) as u32,
    }
}

fn coverage_ppm(rects: &[PixelRect], size: FrameSize) -> u32 {
    let area = union_area(rects, size);
    ppm(area, frame_area(size))
}

pub(super) fn frame_area(size: FrameSize) -> u64 {
    u64::from(size.width) * u64::from(size.height)
}

pub(super) fn ppm(area: u64, total: u64) -> u32 {
    ((area * 1_000_000) / total.max(1)) as u32
}

pub(super) fn union_area(rects: &[PixelRect], size: FrameSize) -> u64 {
    let mut edges = rects
        .iter()
        .flat_map(|rect| {
            [
                rect.x.min(size.width),
                rect.x.saturating_add(rect.width).min(size.width),
            ]
        })
        .collect::<Vec<_>>();
    edges.sort_unstable();
    edges.dedup();
    edges
        .windows(2)
        .map(|edge| strip_area(rects, size, edge[0], edge[1]))
        .sum()
}

fn strip_area(rects: &[PixelRect], size: FrameSize, left: u32, right: u32) -> u64 {
    let mut spans = rects
        .iter()
        .filter(|rect| rect.x < right && rect.x.saturating_add(rect.width) > left)
        .map(|rect| {
            (
                rect.y.min(size.height),
                rect.y.saturating_add(rect.height).min(size.height),
            )
        })
        .filter(|(top, bottom)| bottom > top)
        .collect::<Vec<_>>();
    spans.sort_unstable();
    let (_, height) = spans
        .into_iter()
        .fold((0, 0u64), |(end, total), (top, bottom)| {
            let uncovered = bottom.saturating_sub(top.max(end));
            (end.max(bottom), total + u64::from(uncovered))
        });
    u64::from(right - left) * height
}

#[cfg(test)]
mod tests;
