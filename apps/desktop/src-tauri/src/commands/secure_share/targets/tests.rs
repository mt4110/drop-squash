use super::*;
use dropsquash_core::CaptureRect;

#[test]
fn empty_title_falls_back_to_owner_name() {
    let window = sample_window();
    assert_eq!(dto(&window).title, "Chrome window");
}

#[test]
fn visible_owned_window_remains_selectable_after_recorder_gets_focus() {
    let mut window = sample_window();
    window.active = false;
    assert!(eligible(&&window));
}

fn sample_window() -> SckWindowCandidate {
    SckWindowCandidate {
        window_id: 7,
        title: Some(String::new()),
        owner_name: Some("Chrome".to_string()),
        frame: CaptureRect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        },
        layer: 0,
        on_screen: true,
        active: true,
        has_title: true,
        has_owner: true,
        owner_pid: Some(1),
    }
}
