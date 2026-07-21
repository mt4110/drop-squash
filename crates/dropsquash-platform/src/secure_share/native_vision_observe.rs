use dropsquash_core::{FrameSize, Result, TimeRangeNs, VisionObservation};

use super::native_vision::{decoded_sample_buffer_request_handler, pixel_buffer_request_handler};
use super::{
    perform_text_requests, sample_buffer_request_handler, text_observations,
    text_shape_observations, NativeSampleBuffer,
};
use objc2_core_video::CVPixelBuffer;

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

pub fn observe_pixel_buffer(
    pixel_buffer: &CVPixelBuffer,
    frame_size: FrameSize,
    time_range: TimeRangeNs,
) -> Result<Vec<VisionObservation>> {
    let handler = pixel_buffer_request_handler(pixel_buffer);
    let requests = perform_text_requests(&handler)?;
    let mut observations = text_observations(&requests.text, frame_size, time_range);
    observations.extend(text_shape_observations(
        &requests.text_shape,
        frame_size,
        time_range,
    ));
    Ok(observations)
}

pub(super) fn observe_decoded_sample_buffer(
    sample_buffer: &NativeSampleBuffer,
    frame_size: FrameSize,
) -> Result<Vec<VisionObservation>> {
    let handler = decoded_sample_buffer_request_handler(sample_buffer);
    let requests = perform_text_requests(&handler)?;
    let time_range = TimeRangeNs {
        start_ns: 0,
        end_ns: 1,
    };
    let mut observations = text_observations(&requests.text, frame_size, time_range);
    observations.extend(text_shape_observations(
        &requests.text_shape,
        frame_size,
        time_range,
    ));
    Ok(observations)
}
