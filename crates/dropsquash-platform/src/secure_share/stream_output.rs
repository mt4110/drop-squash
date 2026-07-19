use std::sync::{Arc, Mutex};

use dropsquash_core::{AppError, CaptureFrameMetadata, FrameSize, Result, VisionObservation};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, AnyThread, DefinedClass};
use objc2_foundation::{NSObject, NSObjectProtocol};
use objc2_screen_capture_kit::{SCStream, SCStreamOutput, SCStreamOutputType};

use super::{FrameMetadataProvider, NativeSampleBuffer, SckLiveMaskEvidence};
use crate::secure_share::stream_output_state::SckStreamOutputState;

#[derive(Clone)]
struct StreamOutputIvars {
    state: Arc<Mutex<SckStreamOutputState>>,
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
            if let Ok(mut state) = self.ivars().state.lock() {
                state.capture_sample_buffer(sample_buffer);
            }
        }
    }
);

pub struct SckStreamFrameMetadataOutput {
    object: Retained<SckStreamOutputObject>,
    state: Arc<Mutex<SckStreamOutputState>>,
}

impl SckStreamFrameMetadataOutput {
    pub fn new(frame_size: FrameSize) -> Self {
        let state = Arc::new(Mutex::new(SckStreamOutputState::new(frame_size)));
        let object = SckStreamOutputObject::new(StreamOutputIvars {
            state: Arc::clone(&state),
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

fn lock_state(
    state: &Mutex<SckStreamOutputState>,
) -> Result<std::sync::MutexGuard<'_, SckStreamOutputState>> {
    state.lock().map_err(|_| {
        AppError::InvalidConfig(
            "Secure Share ScreenCaptureKit stream output state is poisoned".to_string(),
        )
    })
}
