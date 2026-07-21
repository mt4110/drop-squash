#[cfg(target_os = "macos")]
mod attachment_rect;
#[cfg(target_os = "macos")]
mod attachment_value;
mod attachments;
#[cfg(target_os = "macos")]
mod ax_capture;
#[cfg(all(test, target_os = "macos"))]
mod ax_capture_tests;
#[cfg(target_os = "macos")]
mod ax_observation;
#[cfg(target_os = "macos")]
mod ax_permission;
#[cfg(target_os = "macos")]
mod ax_probe;
#[cfg(all(test, target_os = "macos"))]
mod ax_probe_tests;
#[cfg(target_os = "macos")]
mod ax_rect;
mod ax_tree;
#[cfg(target_os = "macos")]
mod core_graphics_window;
#[cfg(target_os = "macos")]
mod decoded_residual;
#[cfg(target_os = "macos")]
mod display_watch;
mod exports;
#[cfg(target_os = "macos")]
mod frame_continuity;
mod frame_info;
#[cfg(target_os = "macos")]
mod live_mask;
#[cfg(target_os = "macos")]
mod mach_clock;
#[cfg(target_os = "macos")]
mod native_bridge_accessibility;
#[cfg(target_os = "macos")]
mod native_bridge_metadata;
#[cfg(target_os = "macos")]
mod native_bridge_temporal;
#[cfg(target_os = "macos")]
mod native_bridge_vision;
mod native_frame;
#[cfg(target_os = "macos")]
mod native_vision;
#[cfg(target_os = "macos")]
mod native_vision_observe;
#[cfg(target_os = "macos")]
mod native_vision_request;
#[cfg(target_os = "macos")]
mod native_vision_result;
#[cfg(target_os = "macos")]
mod observation_callback;
#[cfg(target_os = "macos")]
mod observation_probe;
mod output_size;
mod providers;
#[cfg(target_os = "macos")]
mod recording_callback;
#[cfg(target_os = "macos")]
mod recording_probe;
#[cfg(target_os = "macos")]
mod recording_session;
#[cfg(target_os = "macos")]
mod recording_session_callback;
#[cfg(target_os = "macos")]
mod recording_writer;
#[cfg(target_os = "macos")]
mod sample_attachments;
#[cfg(target_os = "macos")]
mod sample_buffer;
#[cfg(target_os = "macos")]
mod sample_buffer_provider;
#[cfg(target_os = "macos")]
mod screen_permission;
#[cfg(target_os = "macos")]
mod selected_window;
#[cfg(target_os = "macos")]
mod shareable_content;
#[cfg(target_os = "macos")]
mod shareable_request;
mod snapshot;
#[cfg(target_os = "macos")]
mod stream_config;
#[cfg(target_os = "macos")]
mod stream_lifecycle;
#[cfg(target_os = "macos")]
mod stream_output;
#[cfg(target_os = "macos")]
mod stream_output_state;
#[cfg(target_os = "macos")]
mod stream_plan;
#[cfg(target_os = "macos")]
mod stream_registration;
#[cfg(target_os = "macos")]
mod target_policy;

pub use exports::*;
#[cfg(target_os = "macos")]
pub use native_bridge_accessibility::accessibility_observations_from_native;
#[cfg(target_os = "macos")]
pub use native_bridge_metadata::capture_frame_metadata_from_native;
#[cfg(target_os = "macos")]
pub use native_bridge_temporal::temporal_observations_from_native;
#[cfg(target_os = "macos")]
pub use native_bridge_vision::vision_observations_from_native;
pub use snapshot::{SecureShareObservationSnapshot, SecureShareProbe, SecureShareSnapshotProvider};

#[cfg(test)]
mod tests;
