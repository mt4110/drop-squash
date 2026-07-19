use dropsquash_core::{FrameSize, Result, TimeRangeNs, VisionObservation};

use super::{
    perform_text_requests, sample_buffer_request_handler, text_observations,
    text_shape_observations, NativeSampleBuffer,
};

pub fn observe_sample_buffer(
    sample_buffer: &NativeSampleBuffer,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
) -> Result<Vec<VisionObservation>> {
    let handler = sample_buffer_request_handler(sample_buffer)?;
    let requests = perform_text_requests(&handler)?;
    let mut observations = text_observations(&requests.text, frame_size, time_range);
    observations.extend(text_shape_observations(
        &requests.text_shape,
        frame_size,
        time_range,
    ));
    Ok(observations)
}
