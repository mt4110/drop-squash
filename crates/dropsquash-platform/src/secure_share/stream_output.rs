use super::{FrameMetadataProvider, NativeSampleBuffer, SckLiveMaskEvidence};
use crate::secure_share::stream_output_state::SckStreamOutputState;
use dropsquash_core::{CaptureFrameMetadata, FrameSize, PixelRect, Result, VisionObservation};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, AnyThread, DefinedClass};
use objc2_foundation::{NSObject, NSObjectProtocol};
use objc2_screen_capture_kit::{SCStream, SCStreamOutput, SCStreamOutputType};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
mod analysis;
mod capture;
mod state;
use state::lock_state;

#[derive(Clone)]
struct StreamOutputIvars {
    state: Arc<Mutex<SckStreamOutputState>>,
    analysis: Arc<analysis::AnalysisGate>,
}

define_class!(
    #[unsafe(super = NSObject)]
    #[ivars = StreamOutputIvars]
    struct SckStreamOutputObject;

    unsafe impl NSObjectProtocol for SckStreamOutputObject {}

    unsafe impl SCStreamOutput for SckStreamOutputObject {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        unsafe fn stream_did_output_sample_buffer(
            &self,
            _stream: &SCStream,
            sample_buffer: &NativeSampleBuffer,
            output_type: SCStreamOutputType,
        ) {
            if output_type != SCStreamOutputType::Screen {
                return;
            }
            capture::frame(&self.ivars().state, &self.ivars().analysis, sample_buffer);
        }
    }
);

pub struct SckStreamFrameMetadataOutput {
    object: Retained<SckStreamOutputObject>,
    state: Arc<Mutex<SckStreamOutputState>>,
}

// State crosses concurrent callbacks and finalization under a mutex.
impl SckStreamFrameMetadataOutput {
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new(frame_size: FrameSize) -> Self {
        let state = Arc::new(Mutex::new(SckStreamOutputState::new(frame_size)));
        let object = SckStreamOutputObject::new(StreamOutputIvars {
            state: Arc::clone(&state),
            analysis: Arc::new(analysis::AnalysisGate::default()),
        });
        Self { object, state }
    }

    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new_recording(
        frame_size: FrameSize,
        output_path: PathBuf,
        fixed_mask_rects: Vec<PixelRect>,
        vision_frame_limit: usize,
    ) -> Self {
        let state = Arc::new(Mutex::new(SckStreamOutputState::with_recording(
            frame_size,
            output_path,
            fixed_mask_rects,
            vision_frame_limit,
        )));
        let object = SckStreamOutputObject::new(StreamOutputIvars {
            state: Arc::clone(&state),
            analysis: Arc::new(analysis::AnalysisGate::default()),
        });
        Self { object, state }
    }

    pub fn as_stream_output(&self) -> &ProtocolObject<dyn SCStreamOutput> {
        ProtocolObject::from_ref(&*self.object)
    }

    pub fn vision_observations(&self) -> Result<Vec<VisionObservation>> {
        lock_state(&self.state).and_then(|state| state.vision_observations())
    }

    pub fn live_mask_evidence(&self) -> Result<SckLiveMaskEvidence> {
        lock_state(&self.state).and_then(|state| state.live_mask_evidence())
    }

    pub fn finish_recording(&self) -> Result<Option<PathBuf>> {
        lock_state(&self.state)?.finish_recording()
    }

    pub fn discard_recording(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.discard_recording();
        }
    }
}

impl FrameMetadataProvider for SckStreamFrameMetadataOutput {
    fn frame_size(&self) -> Result<FrameSize> {
        lock_state(&self.state).map(|state| state.frame_size())
    }

    fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        lock_state(&self.state).and_then(|state| state.frames())
    }
}

impl SckStreamOutputObject {
    fn new(ivars: StreamOutputIvars) -> Retained<Self> {
        let this = Self::alloc().set_ivars(ivars);
        unsafe { msg_send![super(this), init] }
    }
}
