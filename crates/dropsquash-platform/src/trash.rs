use std::path::Path;

use dropsquash_core::{AppError, Result};

#[derive(Debug, Clone, Default)]
pub struct TrashService;

impl TrashService {
    pub fn move_to_trash(&self, _path: &Path) -> Result<()> {
        Err(AppError::InvalidConfig(
            "trash integration is not implemented in Phase 0".to_string(),
        ))
    }
}
