use dispatch2::{DispatchQueue, DispatchQueueAttr, DispatchRetained};
use dropsquash_core::{AppError, CaptureFrameMetadata, FrameSize, Result, VisionObservation};
use objc2::rc::Retained;
use objc2::AnyThread;
use objc2_foundation::NSError;
use objc2_screen_capture_kit::{
    SCContentFilter, SCStream, SCStreamConfiguration, SCStreamOutputType,
};
use std::time::Duration;

use super::{FrameMetadataProvider, SckLiveMaskEvidence, SckStreamFrameMetadataOutput};
use crate::secure_share::stream_lifecycle::{start_stream_capture, stop_stream_capture};
use crate::secure_share::stream_plan::SckStreamCapturePlan;

pub struct SckFrameMetadataStreamRegistration {
    stream: Retained<SCStream>,
    output: SckStreamFrameMetadataOutput,
    _queue: DispatchRetained<DispatchQueue>,
}

impl SckFrameMetadataStreamRegistration {
    pub fn from_plan(plan: &SckStreamCapturePlan) -> Result<Self> {
        Self::new(&plan.filter, &plan.configuration, plan.frame_size)
    }

    pub fn new(
        filter: &SCContentFilter,
        configuration: &SCStreamConfiguration,
        frame_size: FrameSize,
    ) -> Result<Self> {
        let output = SckStreamFrameMetadataOutput::new(frame_size);
        let queue = DispatchQueue::new(
            "app.dropsquash.secure-share.frames",
            DispatchQueueAttr::SERIAL,
        );
        let stream = unsafe {
            SCStream::initWithFilter_configuration_delegate(
                SCStream::alloc(),
                filter,
                configuration,
                None,
            )
        };
        unsafe {
            stream
                .addStreamOutput_type_sampleHandlerQueue_error(
                    output.as_stream_output(),
                    SCStreamOutputType::Screen,
                    Some(&queue),
                )
                .map_err(|error| ns_error("add ScreenCaptureKit stream output", &error))?;
        }
        Ok(Self {
            stream,
            output,
            _queue: queue,
        })
    }

    pub fn stream(&self) -> &SCStream {
        &self.stream
    }

    pub fn start_capture(&self, timeout: Duration) -> Result<()> {
        start_stream_capture(&self.stream, timeout)
    }

    pub fn stop_capture(&self, timeout: Duration) -> Result<()> {
        stop_stream_capture(&self.stream, timeout)
    }

    pub fn observe_for(
        &self,
        capture_duration: Duration,
        completion_timeout: Duration,
    ) -> Result<Vec<CaptureFrameMetadata>> {
        self.start_capture(completion_timeout)?;
        std::thread::sleep(capture_duration);
        self.stop_capture(completion_timeout)?;
        self.frames()
    }

    pub fn vision_observations(&self) -> Result<Vec<VisionObservation>> {
        self.output.vision_observations()
    }

    pub fn live_mask_evidence(&self) -> Result<SckLiveMaskEvidence> {
        self.output.live_mask_evidence()
    }
}

impl FrameMetadataProvider for SckFrameMetadataStreamRegistration {
    fn frame_size(&self) -> Result<FrameSize> {
        self.output.frame_size()
    }

    fn frames(&self) -> Result<Vec<CaptureFrameMetadata>> {
        self.output.frames()
    }
}

fn ns_error(prefix: &str, error: &NSError) -> AppError {
    AppError::InvalidConfig(format!("{prefix}: {}", error.localizedDescription()))
}
