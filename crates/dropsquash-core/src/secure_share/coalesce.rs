use super::{FrameMaskPlan, MaskRegion, PixelRect};

pub(super) fn coalesce_frame_regions(frames: &mut [FrameMaskPlan]) {
    for frame in frames {
        frame.regions = coalesced_regions(std::mem::take(&mut frame.regions));
    }
}

fn coalesced_regions(regions: Vec<MaskRegion>) -> Vec<MaskRegion> {
    let mut merged = Vec::<MaskRegion>::new();
    for region in regions {
        if let Some(existing) = merged
            .iter_mut()
            .find(|existing| can_merge(existing, &region))
        {
            merge_into(existing, region);
        } else {
            merged.push(region);
        }
    }
    merged
}

fn can_merge(left: &MaskRegion, right: &MaskRegion) -> bool {
    left.policy == right.policy
        && left.reason == right.reason
        && left.sources == right.sources
        && expanded(left).touches(expanded(right))
}

fn merge_into(target: &mut MaskRegion, region: MaskRegion) {
    target.rect = target.rect.union(region.rect);
    target.expansion_px = target.expansion_px.max(region.expansion_px);
    target.confidence.detection = target.confidence.detection.max(region.confidence.detection);
    target.confidence.policy = target.confidence.policy.max(region.confidence.policy);
    target.confidence.transform = target.confidence.transform.max(region.confidence.transform);
}

fn expanded(region: &MaskRegion) -> MergeRect {
    MergeRect::from(region.rect).expand(region.expansion_px)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MergeRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl MergeRect {
    fn from(rect: PixelRect) -> Self {
        Self {
            left: rect.x,
            top: rect.y,
            right: rect.x.saturating_add(rect.width),
            bottom: rect.y.saturating_add(rect.height),
        }
    }

    fn expand(self, amount: u32) -> Self {
        Self {
            left: self.left.saturating_sub(amount),
            top: self.top.saturating_sub(amount),
            right: self.right.saturating_add(amount),
            bottom: self.bottom.saturating_add(amount),
        }
    }

    fn touches(self, other: Self) -> bool {
        self.left <= other.right
            && other.left <= self.right
            && self.top <= other.bottom
            && other.top <= self.bottom
    }
}

impl PixelRect {
    fn union(self, other: Self) -> Self {
        let left = self.x.min(other.x);
        let top = self.y.min(other.y);
        let right = self
            .x
            .saturating_add(self.width)
            .max(other.x.saturating_add(other.width));
        let bottom = self
            .y
            .saturating_add(self.height)
            .max(other.y.saturating_add(other.height));
        Self {
            x: left,
            y: top,
            width: right.saturating_sub(left),
            height: bottom.saturating_sub(top),
        }
    }
}
