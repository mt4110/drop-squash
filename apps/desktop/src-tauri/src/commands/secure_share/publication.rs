use std::sync::{Arc, Mutex};

use super::paths::RecordingPaths;

#[derive(Clone, Default)]
pub(super) struct PublicationPermit(Arc<Mutex<bool>>);

#[cfg(test)]
mod tests;

impl PublicationPermit {
    pub(super) fn cancel(&self, paths: &RecordingPaths) {
        if let Ok(mut cancelled) = self.0.lock() {
            *cancelled = true;
        }
        discard(paths);
    }

    pub(super) fn discard(&self, paths: &RecordingPaths) {
        discard(paths);
    }

    pub(super) fn publish(
        &self,
        temporary: &std::path::Path,
        paths: &RecordingPaths,
    ) -> Result<std::path::PathBuf, String> {
        let cancelled = match self.0.lock() {
            Ok(cancelled) => cancelled,
            Err(_) => {
                discard(paths);
                return Err("Secure Share publication state is unavailable".to_string());
            }
        };
        if *cancelled {
            discard(paths);
            return Err("Secure Share recording was cancelled; output was not saved".to_string());
        }
        let sidecar = crate::commands::qa::secure_share::alpha::evidence::publish(
            temporary,
            &paths.final_path,
        )
        .map_err(|error| {
            discard(paths);
            error.user_message()
        })?;
        std::fs::rename(&paths.partial, &paths.final_path).map_err(|error| {
            discard(paths);
            error.to_string()
        })?;
        Ok(sidecar)
    }
}

fn discard(paths: &RecordingPaths) {
    let _ = std::fs::remove_file(&paths.partial);
    let _ = std::fs::remove_file(&paths.final_path);
    let temporary = paths.final_path.with_extension("mask-plan.json.partial");
    crate::commands::qa::secure_share::alpha::evidence::discard(&temporary, &paths.final_path);
}
