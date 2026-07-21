use dropsquash_core::CaptureRect;

use super::super::super::{SckCaptureTarget, SckCaptureTargetKind, SckShareableContentSnapshot};
use super::attested_selection;

#[test]
fn attestation_preserves_the_window_identity_title_and_frame() {
    let target = SckCaptureTarget {
        kind: SckCaptureTargetKind::Window,
        id: 41,
        owner_pid: Some(77),
        frame: rect(10, 20, 640, 480),
    };
    let selection = attested_selection(&snapshot(), 41, target).unwrap();

    assert_eq!(selection.window_id, 41);
    assert_eq!(selection.owner_pid, 77);
    assert_eq!(selection.title.as_deref(), Some("Fixture"));
    assert_eq!(selection.frame, rect(10, 20, 640, 480));
}

fn snapshot() -> SckShareableContentSnapshot {
    SckShareableContentSnapshot {
        windows: vec![super::super::super::SckWindowCandidate {
            window_id: 41,
            owner_pid: Some(77),
            title: Some("Fixture".into()),
            frame: rect(10, 20, 640, 480),
            on_screen: true,
            active: true,
            has_title: true,
            has_owner: true,
            layer: 0,
            owner_name: Some("Fixture.app".into()),
        }],
        displays: Vec::new(),
    }
}

fn rect(x: i32, y: i32, width: u32, height: u32) -> CaptureRect {
    CaptureRect {
        x,
        y,
        width,
        height,
    }
}
