#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeDestructionEvidence {
    pub frame_index: u64,
    pub policy: u32,
    pub region_count: u32,
    pub output_width: u32,
    pub output_height: u32,
}

pub type DestructionEvidenceCallback =
    unsafe extern "C" fn(u64, *const NativeDestructionEvidence, *mut std::ffi::c_void) -> i32;
