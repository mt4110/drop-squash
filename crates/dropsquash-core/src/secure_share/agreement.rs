use super::{AxObservation, TimeRangeNs, VisionObservation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ObservationAgreement {
    pub accessibility_count: usize,
    pub vision_count: usize,
    pub overlapping_pair_count: usize,
    pub accessibility_only_count: usize,
    pub vision_only_count: usize,
}

pub(super) fn measure(
    accessibility: &[AxObservation],
    vision: &[VisionObservation],
) -> ObservationAgreement {
    let pairs = accessibility
        .iter()
        .flat_map(|ax| vision.iter().map(move |item| (ax, item)))
        .filter(|(ax, item)| overlaps(ax.time_range, item.time_range, ax.rect, item.rect))
        .count();
    ObservationAgreement {
        accessibility_count: accessibility.len(),
        vision_count: vision.len(),
        overlapping_pair_count: pairs,
        accessibility_only_count: accessibility
            .iter()
            .filter(|ax| {
                !vision
                    .iter()
                    .any(|item| overlaps(ax.time_range, item.time_range, ax.rect, item.rect))
            })
            .count(),
        vision_only_count: vision
            .iter()
            .filter(|item| {
                !accessibility
                    .iter()
                    .any(|ax| overlaps(ax.time_range, item.time_range, ax.rect, item.rect))
            })
            .count(),
    }
}

fn overlaps(
    left_time: TimeRangeNs,
    right_time: TimeRangeNs,
    left: super::PixelRect,
    right: super::PixelRect,
) -> bool {
    left_time.start_ns < right_time.end_ns
        && right_time.start_ns < left_time.end_ns
        && left.x < right.x.saturating_add(right.width)
        && right.x < left.x.saturating_add(left.width)
        && left.y < right.y.saturating_add(right.height)
        && right.y < left.y.saturating_add(left.height)
}
