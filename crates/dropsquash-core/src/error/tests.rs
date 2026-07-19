use super::AppError;

#[test]
fn rewrites_not_smaller_encoder_error_for_people() {
    let error = AppError::Encoder(
        "native export failed output verification: output is not smaller (1177311 bytes -> 1235958 bytes)".to_string(),
    );

    assert_eq!(
        error.user_message(),
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn keeps_other_errors_unchanged() {
    let error = AppError::UnsupportedMedia("avi".to_string());

    assert_eq!(error.user_message(), "unsupported media: avi");
}

#[test]
fn rewrites_generic_encoder_failure_for_people() {
    let error = AppError::Encoder("The operation could not be completed".to_string());

    assert_eq!(
        error.user_message(),
        "This recording could not be converted, so DropSquash kept the original and did not count the attempt."
    );
}

#[test]
fn rewrites_not_smaller_than_original_variant_for_people() {
    let error = AppError::Encoder("output is not smaller than original".to_string());

    assert_eq!(
        error.user_message(),
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn rewrites_prefixed_not_smaller_encoder_error_for_people() {
    let error = AppError::Encoder(
        "Error: encoder failed: native export failed output verification: output is not smaller (1177311 bytes -> 1235958 bytes)".to_string(),
    );

    assert_eq!(
        error.user_message(),
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn keeps_friendly_not_smaller_message_friendly() {
    let error = AppError::Encoder(
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip.".to_string(),
    );

    assert_eq!(
        error.user_message(),
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
    );
}

#[test]
fn rewrites_license_network_error_for_people() {
    let error = AppError::License("License server is unreachable.".to_string());

    assert_eq!(
        error.user_message(),
        "DropSquash could not reach the license server. Check your connection and try again. Your existing license on this Mac stayed unchanged."
    );
}
