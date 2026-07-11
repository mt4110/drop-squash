use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{OutputSize, Profile, SourcePolicy};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodeJob {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub profile: Profile,
    pub output_size: OutputSize,
    pub source_policy: SourcePolicy,
}
