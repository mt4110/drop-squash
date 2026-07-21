use std::sync::{Mutex, MutexGuard};

use dropsquash_core::{AppError, Result};

use crate::secure_share::stream_output_state::SckStreamOutputState;

pub(super) fn lock_state(
    state: &Mutex<SckStreamOutputState>,
) -> Result<MutexGuard<'_, SckStreamOutputState>> {
    state.lock().map_err(|_| {
        AppError::InvalidConfig(
            "Secure Share ScreenCaptureKit stream output state is poisoned".to_string(),
        )
    })
}
