use super::callbacks::{
    accessibility_callback, destruction_callback, metadata_callback, recording_callback,
    temporal_callback, vision_callback,
};
use super::*;
use std::sync::mpsc::sync_channel;

#[test]
fn callbacks_emit_only_value_events() {
    let (events, receiver) = sync_channel(2);
    let (metadata, metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility_sender, _accessibility_receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility: accessibility_sender,
        temporal,
        destruction,
    });
    let pointer = Arc::into_raw(Arc::clone(&context)).cast_mut().cast();

    unsafe { recording_callback(1, 0, 7, 0, pointer) };
    unsafe { metadata_callback(7, &frame(), pointer) };
    unsafe { recording_callback(2, 0, 7, 1, pointer) };

    assert_eq!(receiver.recv().unwrap(), NativeRecordingEvent::Started);
    assert_eq!(metadata_receiver.recv().unwrap().frame_index, 0);
    assert_eq!(
        receiver.recv().unwrap(),
        NativeRecordingEvent::Completed { frame_count: 1 }
    );
}

#[test]
fn metadata_backpressure_rejects_the_native_session() {
    let (events, receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility, _accessibility_receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility,
        temporal,
        destruction,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(unsafe { metadata_callback(7, &frame(), pointer) }, 0);
    assert_eq!(unsafe { metadata_callback(7, &frame(), pointer) }, 1);
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(
        receiver.recv().unwrap(),
        NativeRecordingEvent::Failed {
            frame_count: 1,
            reason: NativeRecordingFailure::Continuity,
        }
    );
}

#[test]
fn vision_callbacks_emit_only_geometry_values() {
    let (events, _receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision_sender, receiver) = sync_channel(1);
    let (accessibility, _accessibility_receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision: vision_sender,
        accessibility,
        temporal,
        destruction,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(unsafe { vision_callback(7, &vision(), pointer) }, 0);
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(receiver.recv().unwrap(), vision());
}

#[test]
fn accessibility_callbacks_emit_only_geometry_values() {
    let (events, _receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility_sender, receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility: accessibility_sender,
        temporal,
        destruction,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(
        unsafe { accessibility_callback(7, &accessibility(), pointer) },
        0
    );
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(receiver.recv().unwrap(), accessibility());
}

#[test]
fn temporal_callbacks_emit_only_change_geometry() {
    let (events, _receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility, _accessibility_receiver) = sync_channel(1);
    let (temporal_sender, receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility,
        temporal: temporal_sender,
        destruction,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(unsafe { temporal_callback(7, &temporal(), pointer) }, 0);
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(receiver.recv().unwrap(), temporal());
}

#[test]
fn destruction_callbacks_emit_only_writer_facts() {
    let (events, _receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility, _accessibility_receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction_sender, receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility,
        temporal,
        destruction: destruction_sender,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(
        unsafe { destruction_callback(7, &destruction(), pointer) },
        0
    );
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(receiver.recv().unwrap(), destruction());
}

#[test]
fn accessibility_backpressure_rejects_the_native_session() {
    let (events, receiver) = sync_channel(1);
    let (metadata, _metadata_receiver) = sync_channel(1);
    let (vision, _vision_receiver) = sync_channel(1);
    let (accessibility_sender, _accessibility_receiver) = sync_channel(1);
    let (temporal, _temporal_receiver) = sync_channel(1);
    let (destruction, _destruction_receiver) = sync_channel(1);
    let context = Arc::new(CallbackContext {
        events,
        metadata,
        vision,
        accessibility: accessibility_sender,
        temporal,
        destruction,
    });
    let pointer = Arc::into_raw(context).cast_mut().cast();

    assert_eq!(
        unsafe { accessibility_callback(7, &accessibility(), pointer) },
        0
    );
    assert_eq!(
        unsafe { accessibility_callback(7, &accessibility(), pointer) },
        1
    );
    unsafe { recording_callback(3, 6, 7, 1, pointer) };
    assert_eq!(
        receiver.recv().unwrap(),
        NativeRecordingEvent::Failed {
            frame_count: 1,
            reason: NativeRecordingFailure::Continuity
        }
    );
}

fn frame() -> NativeFrameMetadata {
    NativeFrameMetadata {
        frame_index: 0,
        display_time_ticks: 1,
        frame_status: 0,
        scale_factor: 1.0,
        content_scale: 1.0,
        content_x: 0,
        content_y: 0,
        content_width: 2,
        content_height: 2,
        bounding_x: 0,
        bounding_y: 0,
        bounding_width: 2,
        bounding_height: 2,
    }
}

fn vision() -> NativeVisionObservation {
    NativeVisionObservation {
        frame_index: 0,
        kind: 1,
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
        confidence: 0.9,
    }
}

fn accessibility() -> NativeAccessibilityObservation {
    NativeAccessibilityObservation {
        frame_index: 0,
        kind: 1,
        x: 10,
        y: 20,
        width: 30,
        height: 40,
    }
}

fn temporal() -> NativeTemporalObservation {
    NativeTemporalObservation {
        frame_index: 0,
        x: 0.1,
        y: 0.2,
        width: 0.3,
        height: 0.4,
    }
}

fn destruction() -> NativeDestructionEvidence {
    NativeDestructionEvidence {
        frame_index: 0,
        policy: 1,
        region_count: 1,
        output_width: 2,
        output_height: 2,
    }
}
