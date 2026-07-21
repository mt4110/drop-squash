use dropsquash_core::{MaskRect, PixelRect};

const REGION_EXPANSION_PX: u32 = 8;
const ENCODE_GUARD_PX: u32 = 8;

pub(super) fn expanded(regions: &[PixelRect], width: u32, height: u32) -> Vec<MaskRect> {
    let mut output = Vec::new();
    for region in regions
        .iter()
        .filter_map(|region| expand(*region, width, height))
    {
        if let Some(existing) = output
            .iter_mut()
            .find(|existing| touches(**existing, region))
        {
            *existing = union(*existing, region);
        } else {
            output.push(region);
        }
    }
    output
}

fn expand(rect: PixelRect, width: u32, height: u32) -> Option<MaskRect> {
    let expansion = REGION_EXPANSION_PX + ENCODE_GUARD_PX;
    let x = rect.x.saturating_sub(expansion);
    let y = rect.y.saturating_sub(expansion);
    let right = rect
        .x
        .saturating_add(rect.width)
        .saturating_add(expansion)
        .min(width);
    let bottom = rect
        .y
        .saturating_add(rect.height)
        .saturating_add(expansion)
        .min(height);
    (right > x && bottom > y).then_some(MaskRect {
        x,
        y,
        width: right - x,
        height: bottom - y,
    })
}

fn touches(left: MaskRect, right: MaskRect) -> bool {
    left.x <= right.x.saturating_add(right.width)
        && right.x <= left.x.saturating_add(left.width)
        && left.y <= right.y.saturating_add(right.height)
        && right.y <= left.y.saturating_add(left.height)
}

fn union(left: MaskRect, right: MaskRect) -> MaskRect {
    let x = left.x.min(right.x);
    let y = left.y.min(right.y);
    let right_edge = left
        .x
        .saturating_add(left.width)
        .max(right.x.saturating_add(right.width));
    let bottom_edge = left
        .y
        .saturating_add(left.height)
        .max(right.y.saturating_add(right.height));
    MaskRect {
        x,
        y,
        width: right_edge - x,
        height: bottom_edge - y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_nearby_regions_before_writing() {
        let rects = expanded(
            &[
                PixelRect {
                    x: 30,
                    y: 30,
                    width: 20,
                    height: 20,
                },
                PixelRect {
                    x: 60,
                    y: 30,
                    width: 20,
                    height: 20,
                },
            ],
            160,
            90,
        );

        assert_eq!(rects.len(), 1);
    }
}
