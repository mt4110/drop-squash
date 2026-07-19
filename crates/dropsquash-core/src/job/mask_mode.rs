use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskMode {
    SolidBlack,
    BlackNoise,
}

impl MaskMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SolidBlack => "solid_black",
            Self::BlackNoise => "black_noise",
        }
    }
}
