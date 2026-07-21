use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use dropsquash_core::{MaskPolicy, OutputSize};
use objc2::rc::Retained;
use objc2_screen_capture_kit::SCShareableContent;

use crate::secure_share::{SckObservationProbeRequest, SckWindowSelection};

pub(crate) struct RecordingSessionRequest {
    pub(crate) selection: SckWindowSelection,
    pub(crate) output_path: PathBuf,
    pub(crate) output_size: OutputSize,
    pub(crate) policy: MaskPolicy,
    pub(crate) probe: SckObservationProbeRequest,
    pub(crate) content: Retained<SCShareableContent>,
    pub(crate) stop: Receiver<()>,
}
