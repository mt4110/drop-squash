mod backend;
mod gstreamer;
mod media_foundation;
mod verify;
mod videotoolbox;

pub use backend::{EncodeProgressReporter, EncoderBackend, EncoderCapabilities};
pub use gstreamer::GStreamerEncoder;
pub use media_foundation::MediaFoundationEncoder;
pub use verify::{verify_output, OutputVerification};
pub use videotoolbox::VideoToolboxEncoder;

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
}
