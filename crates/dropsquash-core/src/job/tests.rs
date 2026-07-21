use std::path::PathBuf;

use serde_json::json;

use super::{EncodeJob, MaskMode, MaskRect, OutputSize, Profile, SecureShareOptions, SourcePolicy};

#[test]
fn secure_share_payload_uses_stable_wire_names() {
    let options = SecureShareOptions {
        mask_mode: MaskMode::BlackNoise,
        mask_rects: vec![MaskRect {
            x: 12,
            y: 24,
            width: 320,
            height: 180,
        }],
        mask_plan: None,
    };

    let value = serde_json::to_value(&options).unwrap();

    assert_eq!(
        value,
        json!({
            "maskMode": "black_noise",
            "maskRects": [{ "x": 12, "y": 24, "width": 320, "height": 180 }]
        })
    );
}

#[test]
fn encode_job_omits_secure_share_when_absent() {
    let job = EncodeJob {
        input_path: PathBuf::from("input.mov"),
        output_dir: PathBuf::from("/tmp/out"),
        profile: Profile::Auto,
        output_size: OutputSize::Auto,
        source_policy: SourcePolicy::Keep,
        secure_share: None,
    };

    let value = serde_json::to_value(&job).unwrap();

    assert_eq!(value.get("secure_share"), None);
}

#[test]
fn encode_job_round_trips_secure_share_payload() {
    let value = json!({
        "input_path": "input.mov",
        "output_dir": "/tmp/out",
        "profile": "auto",
        "output_size": "auto",
        "source_policy": "keep",
        "secure_share": {
            "maskMode": "solid_black",
            "maskRects": [{ "x": 1, "y": 2, "width": 3, "height": 4 }]
        }
    });

    let job: EncodeJob = serde_json::from_value(value).unwrap();
    let payload = serde_json::to_value(&job).unwrap();

    assert_eq!(
        payload.get("secure_share"),
        Some(&json!({
            "maskMode": "solid_black",
            "maskRects": [{ "x": 1, "y": 2, "width": 3, "height": 4 }]
        }))
    );
}
