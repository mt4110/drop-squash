use dropsquash_core::FrameSize;
use objc2::rc::Retained;
use objc2_screen_capture_kit::SCStreamConfiguration;

pub fn stream_configuration(frame_size: FrameSize) -> Retained<SCStreamConfiguration> {
    let configuration = unsafe { SCStreamConfiguration::new() };
    unsafe {
        configuration.setWidth(frame_size.width as usize);
        configuration.setHeight(frame_size.height as usize);
        configuration.setPixelFormat(fourcc("BGRA"));
        configuration.setQueueDepth(3);
        configuration.setShowsCursor(false);
        configuration.setCapturesAudio(false);
        configuration.setScalesToFit(false);
        configuration.setPreservesAspectRatio(true);
    }
    configuration
}

fn fourcc(value: &str) -> u32 {
    u32::from_be_bytes(value.as_bytes().try_into().unwrap())
}
