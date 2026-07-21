use dropsquash_core::OutputSize;

pub(super) fn dimensions(
    width: u32,
    height: u32,
    output: OutputSize,
) -> Result<(u32, u32), String> {
    let limit = match output {
        OutputSize::Auto => return even(width, height),
        OutputSize::P1080 => (1920, 1080),
        OutputSize::P720 => (1280, 720),
        OutputSize::P480 => (640, 480),
    };
    let ratio = (limit.0 as f64 / width as f64)
        .min(limit.1 as f64 / height as f64)
        .min(1.0);
    even(
        (width as f64 * ratio) as u32,
        (height as f64 * ratio) as u32,
    )
}

fn even(width: u32, height: u32) -> Result<(u32, u32), String> {
    let width = width / 2 * 2;
    let height = height / 2 * 2;
    (width >= 2 && height >= 2)
        .then_some((width, height))
        .ok_or_else(|| "Secure Share selected window is too small".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_auto_dimensions_without_upscaling() {
        assert_eq!(dimensions(801, 603, OutputSize::Auto).unwrap(), (800, 602));
    }

    #[test]
    fn fits_wide_windows_inside_the_requested_size() {
        assert_eq!(
            dimensions(3840, 2160, OutputSize::P720).unwrap(),
            (1280, 720)
        );
    }

    #[test]
    fn fits_tall_windows_without_distortion() {
        assert_eq!(dimensions(900, 1800, OutputSize::P480).unwrap(), (240, 480));
    }
}
