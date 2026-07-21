use dropsquash_core::{MaskRect, PixelRect};

pub(super) fn contains(outer: MaskRect, inner: PixelRect) -> bool {
    outer.x <= inner.x
        && outer.y <= inner.y
        && outer.x.saturating_add(outer.width) >= inner.x.saturating_add(inner.width)
        && outer.y.saturating_add(outer.height) >= inner.y.saturating_add(inner.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_the_full_text_box_to_be_masked() {
        let mask = MaskRect {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };
        assert!(contains(
            mask,
            PixelRect {
                x: 12,
                y: 12,
                width: 10,
                height: 10
            }
        ));
        assert!(!contains(
            mask,
            PixelRect {
                x: 28,
                y: 12,
                width: 5,
                height: 5
            }
        ));
    }
}
