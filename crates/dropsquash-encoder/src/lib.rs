mod backend;
mod gstreamer;
mod media_foundation;
mod verify;
mod videotoolbox;

pub use backend::{EncodeProgressReporter, EncoderBackend, EncoderCapabilities};
pub use gstreamer::GStreamerEncoder;
pub use media_foundation::MediaFoundationEncoder;
pub use verify::{verify_output, OutputVerification};
pub use videotoolbox::AppleNativeEncoder;

#[cfg(test)]
pub use backend::NoopEncoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unimplemented_platform_backends_report_unavailable() {
        let backends: [&dyn EncoderBackend; 2] = [&MediaFoundationEncoder, &GStreamerEncoder];

        for backend in backends {
            let capabilities = backend.probe_capabilities().unwrap();
            assert_eq!(capabilities.backend_name, backend.name());
            assert!(!capabilities.available);
            assert!(!capabilities.hardware_acceleration);
            assert!(!capabilities.supports_h264);
            assert!(!capabilities.supports_hevc);
        }
    }

    #[tokio::test]
    async fn unimplemented_platform_backends_reject_encode() {
        let output_dir = tempfile::tempdir().unwrap();
        let backends: [&dyn EncoderBackend; 2] = [&MediaFoundationEncoder, &GStreamerEncoder];

        for backend in backends {
            let error = backend
                .encode(dropsquash_core::EncodeJob {
                    input_path: "input.mov".into(),
                    output_dir: output_dir.path().to_path_buf(),
                    profile: dropsquash_core::Profile::Auto,
                    output_size: dropsquash_core::OutputSize::Auto,
                    source_policy: dropsquash_core::SourcePolicy::Keep,
                })
                .await
                .unwrap_err();
            assert!(error.to_string().contains("cannot use the native encoder"));
        }
    }
}
