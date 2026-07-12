use std::path::Path;

use dropsquash_core::AppError;
use dropsquash_encoder::EncoderBackend;

use super::{format_error, NativeEncoder};

pub(super) fn ensure_supported(input_path: &Path) -> Result<(), String> {
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;
    let extension = normalized_extension(input_path);
    if is_supported_extension(&capabilities.input_extensions, &extension) {
        return Ok(());
    }
    Err(format_error(AppError::UnsupportedMedia(extension)))
}

fn normalized_extension(input_path: &Path) -> String {
    input_path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default()
}

fn is_supported_extension(allowed: &[String], extension: &str) -> bool {
    allowed.iter().any(|candidate| candidate == extension)
}

#[cfg(test)]
mod tests {
    use super::{is_supported_extension, normalized_extension};

    #[test]
    fn normalizes_extensions_to_lowercase() {
        assert_eq!(
            normalized_extension(std::path::Path::new("Clip.MOV")),
            "mov"
        );
    }

    #[test]
    fn rejects_missing_extension() {
        assert_eq!(normalized_extension(std::path::Path::new("Clip")), "");
    }

    #[test]
    fn matches_supported_extensions_exactly() {
        let allowed = vec!["mov".to_string(), "mp4".to_string()];

        assert!(is_supported_extension(&allowed, "mov"));
        assert!(!is_supported_extension(&allowed, "avi"));
    }
}
