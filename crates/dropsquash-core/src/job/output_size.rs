use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputSize {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1080p")]
    P1080,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "480p")]
    P480,
}

impl OutputSize {
    pub const ALL: [Self; 4] = [Self::Auto, Self::P1080, Self::P720, Self::P480];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::P1080 => "1080p",
            Self::P720 => "720p",
            Self::P480 => "480p",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::P1080 => "1920 x 1080",
            Self::P720 => "1280 x 720",
            Self::P480 => "640 x 480",
        }
    }
}

impl FromStr for OutputSize {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "1080p" => Ok(Self::P1080),
            "720p" => Ok(Self::P720),
            "480p" => Ok(Self::P480),
            other => Err(format!("unknown output size: {other}")),
        }
    }
}
