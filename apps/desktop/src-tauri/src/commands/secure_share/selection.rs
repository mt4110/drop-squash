use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureShareWindowSelectionDto {
    window_id: u32,
    owner_pid: i32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl SecureShareWindowSelectionDto {
    pub(super) fn into_native(self) -> NativeWindowSelection {
        NativeWindowSelection {
            window_id: self.window_id,
            owner_pid: self.owner_pid,
            frame: dropsquash_core::CaptureRect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
            },
        }
    }
}

pub(super) struct NativeWindowSelection {
    pub(super) window_id: u32,
    pub(super) owner_pid: i32,
    pub(super) frame: dropsquash_core::CaptureRect,
}
