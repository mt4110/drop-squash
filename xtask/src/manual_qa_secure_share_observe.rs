use std::time::Duration;

mod args;

use args::{parse_args, Mode, Options};

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let options = parse_args(args)?;
    run_probe(options)
}

#[cfg(target_os = "macos")]
fn run_probe(options: Options) -> Result<(), String> {
    use dropsquash_platform::{
        observe_strict_reveal_window_once, observe_window_once, request_shareable_content_snapshot,
        SckObservationProbeRequest, SckShareableContentRequest,
    };
    if options.mode == Mode::List {
        let snapshot = request_shareable_content_snapshot(SckShareableContentRequest {
            exclude_desktop_windows: true,
            on_screen_windows_only: true,
            timeout: Duration::from_millis(options.timeout_ms),
        })
        .map_err(|error| error.to_string())?;
        println!("secure share windows: {}", snapshot.windows.len());
        for window in snapshot.windows {
            println!("window {} {:?}", window.window_id, window.frame);
        }
        println!("secure share displays: {}", snapshot.displays.len());
        for display in snapshot.displays {
            println!("display {} {:?}", display.display_id, display.frame);
        }
        return Ok(());
    }
    require_gui_hosted_capture()?;
    let request = SckObservationProbeRequest {
        discovery_timeout: Duration::from_millis(options.timeout_ms),
        completion_timeout: Duration::from_millis(options.timeout_ms),
        capture_duration: Duration::from_millis(options.capture_ms),
    };
    let report = match options.mode {
        Mode::Auto => observe_strict_reveal_window_once(request),
        Mode::Window(id) => observe_window_once(id, request),
        Mode::List => unreachable!(),
    }
    .map_err(|error| error.to_string())?;
    println!("secure share target: {:?}", report.target);
    println!(
        "secure share frame size: {}x{}",
        report.frame_size.width, report.frame_size.height
    );
    println!("secure share frame count: {}", report.frames.len());
    println!(
        "secure share ax observation count: {}",
        report.accessibility.len()
    );
    println!(
        "secure share vision observation count: {}",
        report.vision.len()
    );
    for frame in report.frames.iter().take(5) {
        println!(
            "frame {}: t={}ns status={:?}",
            frame.frame_index, frame.presentation_time_ns, frame.frame_status
        );
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn require_gui_hosted_capture() -> Result<(), String> {
    Err(
        "ScreenCaptureKit capture observation must run from a GUI-hosted harness; \
         CLI capture start is disabled because CoreGraphics can abort before DropSquash \
         can fail closed. Use `manual-qa-secure-share-observe list` only."
            .to_string(),
    )
}

#[cfg(not(target_os = "macos"))]
fn run_probe(_options: Options) -> Result<(), String> {
    Err("manual-qa-secure-share-observe requires macOS".to_string())
}
