#[cfg(target_os = "macos")]
pub type DiscoveryCallback = unsafe extern "C" fn(i32, i32, i32, *mut std::ffi::c_void);

#[cfg(target_os = "macos")]
pub type FrameCallback =
    unsafe extern "C" fn(i32, u32, u32, u32, *mut u8, usize, *mut std::ffi::c_void);

#[cfg(target_os = "macos")]
mod recording;
#[cfg(target_os = "macos")]
pub use recording::{
    start_attested_native_strict_recording, start_attested_strict_recording_with_observations,
    start_attested_strict_recording_with_temporal_observations, start_native_strict_recording,
    start_strict_recording, start_strict_recording_with_metadata, stop_strict_recording,
    AttestedObservationCallbacks, AttestedRecordingRequest, AttestedTemporalCallbacks,
    DestructionEvidenceCallback, FrameMetadataCallback, NativeAccessibilityObservation,
    NativeDestructionEvidence, NativeFrameMetadata, NativeRecordingEvent, NativeRecordingFailure,
    NativeStrictRecordingHandle, NativeTemporalObservation, NativeVisionObservation,
    RecordingCallback, TemporalObservationCallback, VisionObservationCallback,
};

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn dropsquash_secure_share_bridge_abi_version() -> i32;
    fn dropsquash_secure_share_bridge_discover(
        context: *mut std::ffi::c_void,
        callback: DiscoveryCallback,
    );
    fn dropsquash_secure_share_bridge_capture_window_frame(
        window_id: u32,
        strict_shield: i32,
        context: *mut std::ffi::c_void,
        callback: FrameCallback,
    );
    fn dropsquash_secure_share_bridge_release_frame(bytes: *mut u8);
}

pub fn apple_capture_bridge_abi_version() -> i32 {
    #[cfg(target_os = "macos")]
    unsafe {
        dropsquash_secure_share_bridge_abi_version()
    }
    #[cfg(not(target_os = "macos"))]
    0
}

#[cfg(target_os = "macos")]
/// # Safety
///
/// `context` must remain valid until the native callback receives it.
pub unsafe fn discover_secure_share_content(
    context: *mut std::ffi::c_void,
    callback: DiscoveryCallback,
) {
    unsafe { dropsquash_secure_share_bridge_discover(context, callback) };
}

#[cfg(target_os = "macos")]
/// # Safety
///
/// `context` must remain valid until the native callback receives it. The
/// callback must release a successful `bytes` pointer exactly once.
pub unsafe fn capture_window_frame(
    window_id: u32,
    strict_shield: bool,
    context: *mut std::ffi::c_void,
    callback: FrameCallback,
) {
    unsafe {
        dropsquash_secure_share_bridge_capture_window_frame(
            window_id,
            i32::from(strict_shield),
            context,
            callback,
        )
    };
}

#[cfg(target_os = "macos")]
/// # Safety
///
/// `bytes` must be a successful frame pointer received from this bridge.
pub unsafe fn release_copied_frame(bytes: *mut u8) {
    unsafe { dropsquash_secure_share_bridge_release_frame(bytes) };
}

#[cfg(test)]
mod tests {
    use super::apple_capture_bridge_abi_version;

    #[test]
    fn exports_the_expected_abi_version() {
        assert_eq!(apple_capture_bridge_abi_version(), 1);
    }
}
