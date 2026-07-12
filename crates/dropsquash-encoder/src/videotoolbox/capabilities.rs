use crate::EncoderCapabilities;

pub(super) fn probe(backend_name: &str) -> EncoderCapabilities {
    #[cfg(target_os = "macos")]
    {
        EncoderCapabilities {
            backend_name: backend_name.to_string(),
            available: true,
            hardware_acceleration: false,
            supports_h264: true,
            supports_hevc: false,
            supports_metadata_strip: false,
            input_extensions: vec!["mov".to_string(), "mp4".to_string(), "m4v".to_string()],
        }
    }

    #[cfg(not(target_os = "macos"))]
    EncoderCapabilities {
        backend_name: backend_name.to_string(),
        available: false,
        hardware_acceleration: false,
        supports_h264: false,
        supports_hevc: false,
        supports_metadata_strip: false,
        input_extensions: Vec::new(),
    }
}
