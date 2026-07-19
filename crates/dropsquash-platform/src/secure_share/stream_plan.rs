use dropsquash_core::{AppError, FrameSize, Result};
use objc2::rc::Retained;
use objc2::AnyThread;
use objc2_foundation::NSArray;
use objc2_screen_capture_kit::{
    SCContentFilter, SCDisplay, SCShareableContent, SCStreamConfiguration, SCWindow,
};

use super::{SckCaptureTarget, SckCaptureTargetKind};
use crate::secure_share::stream_config::stream_configuration;

pub struct SckStreamCapturePlan {
    pub filter: Retained<SCContentFilter>,
    pub configuration: Retained<SCStreamConfiguration>,
    pub frame_size: FrameSize,
}

pub fn build_stream_capture_plan(
    content: &SCShareableContent,
    target: &SckCaptureTarget,
) -> Result<SckStreamCapturePlan> {
    let filter = match target.kind {
        SckCaptureTargetKind::Window => window_filter(content, target.id)?,
        SckCaptureTargetKind::Display => display_filter(content, target.id)?,
    };
    let frame_size = frame_size_from_filter(&filter)?;
    let configuration = stream_configuration(frame_size);
    Ok(SckStreamCapturePlan {
        filter,
        configuration,
        frame_size,
    })
}

fn window_filter(
    content: &SCShareableContent,
    window_id: u32,
) -> Result<Retained<SCContentFilter>> {
    let window = find_one_window(content, window_id)?;
    Ok(unsafe {
        SCContentFilter::initWithDesktopIndependentWindow(SCContentFilter::alloc(), &window)
    })
}

fn display_filter(
    content: &SCShareableContent,
    display_id: u32,
) -> Result<Retained<SCContentFilter>> {
    let display = find_one_display(content, display_id)?;
    let excluded = NSArray::<SCWindow>::from_slice(&[]);
    Ok(unsafe {
        SCContentFilter::initWithDisplay_excludingWindows(
            SCContentFilter::alloc(),
            &display,
            &excluded,
        )
    })
}

fn frame_size_from_filter(filter: &SCContentFilter) -> Result<FrameSize> {
    let rect = unsafe { filter.contentRect() };
    let scale = unsafe { filter.pointPixelScale() };
    if !scale.is_finite() || scale <= 0.0 {
        return Err(invalid("target scale is invalid"));
    }
    Ok(FrameSize {
        width: scaled_dimension(rect.size.width, scale, "target width")?,
        height: scaled_dimension(rect.size.height, scale, "target height")?,
    })
}

fn find_one_window(content: &SCShareableContent, id: u32) -> Result<Retained<SCWindow>> {
    let matches = unsafe { content.windows() }
        .to_vec()
        .into_iter()
        .filter(|window| unsafe { window.windowID() } == id)
        .collect::<Vec<_>>();
    one(matches, "window")
}

fn find_one_display(content: &SCShareableContent, id: u32) -> Result<Retained<SCDisplay>> {
    let matches = unsafe { content.displays() }
        .to_vec()
        .into_iter()
        .filter(|display| unsafe { display.displayID() } == id)
        .collect::<Vec<_>>();
    one(matches, "display")
}

fn one<T>(items: Vec<T>, name: &str) -> Result<T> {
    match items.len() {
        1 => Ok(items.into_iter().next().unwrap()),
        0 => Err(invalid(&format!("target {name} is missing"))),
        _ => Err(invalid(&format!("target {name} is ambiguous"))),
    }
}

fn scaled_dimension(points: f64, scale: f32, name: &str) -> Result<u32> {
    let pixels = points * f64::from(scale);
    if !pixels.is_finite() || pixels <= 0.0 || pixels > u32::MAX as f64 {
        return Err(invalid(&format!("{name} is invalid")));
    }
    Ok(pixels.round() as u32)
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!(
        "Secure Share ScreenCaptureKit stream plan {reason}"
    ))
}
