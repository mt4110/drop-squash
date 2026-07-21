use dropsquash_core::AppError;
use objc2_av_foundation::AVAssetWriter;
use objc2_foundation::NSError;

pub(super) fn writer_status(prefix: &str, writer: &AVAssetWriter) -> AppError {
    unsafe { writer.error() }.map_or_else(
        || AppError::Encoder(format!("Secure Share recording {prefix}")),
        |error| writer_error(prefix, &error),
    )
}

pub(super) fn writer_error(prefix: &str, error: &NSError) -> AppError {
    AppError::Encoder(format!(
        "Secure Share recording {prefix}: {}",
        error.localizedDescription()
    ))
}

pub(super) fn missing_format() -> AppError {
    AppError::UnsupportedMedia("Secure Share recording frame format is missing".into())
}

pub(super) fn missing_file_type() -> AppError {
    AppError::Encoder("Secure Share MPEG-4 output is unavailable".into())
}

pub(super) fn missing_codec() -> AppError {
    AppError::Encoder("Secure Share H.264 output is unavailable".into())
}
