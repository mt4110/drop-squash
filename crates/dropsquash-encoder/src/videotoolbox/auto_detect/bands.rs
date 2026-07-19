use dropsquash_core::MaskRect;

pub(super) fn detect_row_rects(
    width: u32,
    height: u32,
    motion: &[u32],
    edge: &[u8],
    compared: u32,
) -> Vec<MaskRect> {
    let mut rects = Vec::new();
    let mut start = None;
    for y in 0..height as usize {
        let stable = compared > 0 && motion[y] / compared <= 6;
        let detailed = edge[y] >= 10;
        match (stable && detailed, start) {
            (true, None) => start = Some(y as u32),
            (false, Some(band_start)) => push_band(&mut rects, width, height, band_start, y as u32),
            _ => {}
        }
        if start.is_some() && !(stable && detailed) {
            start = None;
        }
    }
    if let Some(band_start) = start {
        push_band(&mut rects, width, height, band_start, height);
    }
    rects.into_iter().take(2).collect()
}

fn push_band(rects: &mut Vec<MaskRect>, width: u32, height: u32, start: u32, end: u32) {
    let band_height = end.saturating_sub(start);
    let near_edge = start < height / 3 || end > height.saturating_mul(2) / 3;
    if near_edge && (24..=220).contains(&band_height) {
        rects.push(MaskRect {
            x: 0,
            y: start,
            width,
            height: band_height,
        });
    }
}
