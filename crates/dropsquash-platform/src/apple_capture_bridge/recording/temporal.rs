use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeTemporalObservation {
    pub frame_index: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub type TemporalObservationCallback =
    unsafe extern "C" fn(u64, *const NativeTemporalObservation, *mut c_void) -> i32;
