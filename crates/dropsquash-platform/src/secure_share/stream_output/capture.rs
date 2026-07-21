use std::sync::Mutex;
use std::time::Instant;

use crate::secure_share::stream_output_state::{frame_time_range, SckStreamOutputState};
use crate::secure_share::NativeSampleBuffer;

use super::analysis::AnalysisGate;

pub(super) fn frame(
    state: &Mutex<SckStreamOutputState>,
    analysis: &AnalysisGate,
    sample: &NativeSampleBuffer,
) {
    let Some((frame, size, needs_vision)) = state.lock().ok().and_then(|mut state| {
        state
            .admit_sample_buffer(sample)
            .map(|frame| (frame, state.vision_input()))
            .map(|(frame, (size, needs_vision))| (frame, size, needs_vision))
    }) else {
        return;
    };
    let started = Instant::now();
    let slot = needs_vision.then(|| analysis.try_acquire(size)).flatten();
    if needs_vision && slot.is_none() {
        reject_capacity(state);
        return;
    }
    let vision = needs_vision
        .then(|| {
            super::super::native_vision_observe::observe_sample_buffer(
                sample,
                size,
                frame_time_range(frame.presentation_time_ns),
            )
        })
        .transpose()
        .map(|value| value.unwrap_or_default());
    if let Ok(mut state) = state.lock() {
        state.capture_admitted_sample_buffer(
            sample,
            frame,
            vision,
            needs_vision.then(|| started.elapsed()),
        );
    }
}

fn reject_capacity(state: &Mutex<SckStreamOutputState>) {
    if let Ok(mut state) = state.lock() {
        state.reject_analysis_capacity();
    }
}
