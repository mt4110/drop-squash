mod backend;
mod experimental;
mod gstreamer;
mod media_foundation;
mod secure_share;
mod verify;
mod videotoolbox;

pub use backend::{EncodeProgressReporter, EncoderBackend, EncoderCapabilities};
pub use experimental::verify_experimental_mask_plan_output;
pub use gstreamer::GStreamerEncoder;
pub use media_foundation::MediaFoundationEncoder;
pub use secure_share::{
    prove_mask_plan_solid_black_fill, resolve_secure_share_options, verify_mask_plan_output,
    MaskPlanBlackFillProof,
};
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
                    secure_share: None,
                })
                .await
                .unwrap_err();
            let message = error.to_string();
            assert!(message.contains("native-encoder-unavailable"));
            assert!(message.contains(backend.name()));
        }
    }
}
