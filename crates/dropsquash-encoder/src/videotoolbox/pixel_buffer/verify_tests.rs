use dropsquash_core::MaskRect;

use super::verify_bgra_regions;

#[test]
fn accepts_black_masked_pixels_with_padded_rows() {
    let frame = vec![0, 0, 0, 255, 0, 0, 0, 255, 99, 99, 99, 99];

    verify_bgra_regions(
        &frame,
        2,
        1,
        12,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 2,
            height: 1,
        }],
    )
    .unwrap();
}

#[test]
fn accepts_neutral_limited_range_black() {
    let frame = [16, 16, 16, 255];

    verify_bgra_regions(
        &frame,
        1,
        1,
        4,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }],
    )
    .unwrap();
}

#[test]
fn accepts_encoded_black_with_small_chroma_quantization() {
    let frame = [0, 5, 0, 255];

    verify_bgra_regions(
        &frame,
        1,
        1,
        4,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }],
    )
    .unwrap();
}

#[test]
fn rejects_residual_non_black_pixel_inside_mask_plan() {
    let frame = [0, 0, 0, 255, 33, 0, 0, 255];

    let error = verify_bgra_regions(
        &frame,
        2,
        1,
        8,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 2,
            height: 1,
        }],
    )
    .unwrap_err();

    assert!(error.to_string().contains("non-black masked output pixels"));
}

#[test]
fn rejects_dark_colored_residue_inside_mask_plan() {
    let frame = [0, 0, 7, 255];

    let error = verify_bgra_regions(
        &frame,
        1,
        1,
        4,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }],
    )
    .unwrap_err();

    assert!(error.to_string().contains("non-black masked output pixels"));
}

#[test]
fn rejects_dim_neutral_residue_that_can_be_contrast_amplified() {
    let frame = [24, 24, 24, 255];

    let error = verify_bgra_regions(
        &frame,
        1,
        1,
        4,
        &[MaskRect {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }],
    )
    .unwrap_err();

    assert!(error.to_string().contains("non-black masked output pixels"));
}
