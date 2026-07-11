use std::sync::Mutex;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
pub struct AppState {
    pub config_lock: Mutex<()>,
    pub active_conversion: Mutex<Option<CancellationToken>>,
}

impl AppState {
    pub fn start_conversion(&self) -> Result<CancellationToken, String> {
        let mut active = self.active_conversion.lock().map_err(lock_error)?;
        if active.is_some() {
            return Err("A conversion is already running.".to_string());
        }
        let token = CancellationToken::new();
        *active = Some(token.clone());
        Ok(token)
    }

    pub fn cancel_conversion(&self) -> Result<bool, String> {
        let active = self.active_conversion.lock().map_err(lock_error)?;
        Ok(active.as_ref().is_some_and(|token| {
            token.cancel();
            true
        }))
    }

    pub fn finish_conversion(&self) {
        if let Ok(mut active) = self.active_conversion.lock() {
            *active = None;
        }
    }
}

fn lock_error<T>(_error: T) -> String {
    "conversion state lock was poisoned".to_string()
}

#[cfg(test)]
mod tests;
