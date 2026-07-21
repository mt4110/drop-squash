use dropsquash_core::{FrameSize, OutputSize};

pub(super) fn final_frame_size(source: FrameSize, output_size: OutputSize) -> FrameSize {
    let Some(max_edge) = max_edge(output_size) else {
        return even_frame_size(source);
    };
    let longest = source.width.max(source.height);
    if longest <= max_edge {
        return even_frame_size(source);
    }
    let scale = f64::from(max_edge) / f64::from(longest);
    FrameSize {
        width: even(f64::from(source.width) * scale, max_edge),
        height: even(f64::from(source.height) * scale, max_edge),
    }
}

fn even_frame_size(source: FrameSize) -> FrameSize {
    FrameSize {
        width: even(f64::from(source.width), source.width),
        height: even(f64::from(source.height), source.height),
    }
}

fn max_edge(output_size: OutputSize) -> Option<u32> {
    match output_size {
        OutputSize::Auto => None,
        OutputSize::P1080 => Some(1920),
        OutputSize::P720 => Some(1280),
        OutputSize::P480 => Some(640),
    }
}

fn even(value: f64, max: u32) -> u32 {
    let rounded = value.round().clamp(2.0, f64::from(max)) as u32;
    let even = rounded - (rounded % 2);
    even.max(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_auto_and_normalizes_to_even_dimensions() {
        let source = FrameSize {
            width: 601,
            height: 401,
        };
        assert_eq!(
            final_frame_size(source, OutputSize::Auto),
            FrameSize {
                width: 600,
                height: 400
            }
        );
        assert_eq!(
            final_frame_size(source, OutputSize::P480),
            FrameSize {
                width: 600,
                height: 400
            }
        );
    }

    #[test]
    fn scales_portrait_to_an_even_final_size() {
        assert_eq!(
            final_frame_size(
                FrameSize {
                    width: 2160,
                    height: 3840
                },
                OutputSize::P720
            ),
            FrameSize {
                width: 720,
                height: 1280
            }
        );
    }
}
