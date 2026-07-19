use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationExpectations {
    pub no_audio: bool,
    pub strip_metadata: bool,
    pub verification_policy_version: String,
}
