use super::{AxObservation, AxObservationKind, VisionObservation};

pub(super) fn refined_accessibility(
    accessibility: Vec<AxObservation>,
    vision: &[VisionObservation],
) -> Vec<AxObservation> {
    accessibility
        .into_iter()
        .filter(|item| !is_multiline_container(*item, vision))
        .collect()
}

fn is_multiline_container(item: AxObservation, vision: &[VisionObservation]) -> bool {
    item.kind == AxObservationKind::TextElement
        && vision
            .iter()
            .filter(|candidate| overlaps(item, **candidate))
            .take(2)
            .count()
            == 2
}

fn overlaps(left: AxObservation, right: VisionObservation) -> bool {
    left.time_range.start_ns < right.time_range.end_ns
        && right.time_range.start_ns < left.time_range.end_ns
        && left.rect.x < right.rect.x.saturating_add(right.rect.width)
        && right.rect.x < left.rect.x.saturating_add(left.rect.width)
        && left.rect.y < right.rect.y.saturating_add(right.rect.height)
        && right.rect.y < left.rect.y.saturating_add(left.rect.height)
}

#[cfg(test)]
mod tests;
