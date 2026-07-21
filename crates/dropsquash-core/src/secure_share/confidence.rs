use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Confidence {
    pub detection: f32,
    pub transform: f32,
    pub policy: f32,
}

impl Confidence {
    pub const CERTAIN: Self = Self {
        detection: 1.0,
        transform: 1.0,
        policy: 1.0,
    };
}
