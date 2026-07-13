use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) fn artifact_digest(path: &Path) -> Result<String, String> {
    let bytes = crate::dmg::read(path, "manual QA checksum artifact")?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
