use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    Auto,
    Slack,
    Teams,
    Discord,
    Chatwork,
    Line,
    #[serde(rename = "whatsapp")]
    WhatsApp,
    Docs,
    Archive,
    Privacy,
}

impl Profile {
    pub const ALL: [Self; 10] = [
        Self::Auto,
        Self::Slack,
        Self::Teams,
        Self::Discord,
        Self::Chatwork,
        Self::Line,
        Self::WhatsApp,
        Self::Docs,
        Self::Archive,
        Self::Privacy,
    ];

    pub const DELIVERY: [Self; 8] = [
        Self::Auto,
        Self::Slack,
        Self::Teams,
        Self::Discord,
        Self::Chatwork,
        Self::Line,
        Self::WhatsApp,
        Self::Docs,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Slack => "slack",
            Self::Teams => "teams",
            Self::Discord => "discord",
            Self::Chatwork => "chatwork",
            Self::Line => "line",
            Self::WhatsApp => "whatsapp",
            Self::Docs => "docs",
            Self::Archive => "archive",
            Self::Privacy => "privacy",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Slack => "Slack",
            Self::Teams => "Microsoft Teams",
            Self::Discord => "Discord",
            Self::Chatwork => "Chatwork",
            Self::Line => "LINE",
            Self::WhatsApp => "WhatsApp",
            Self::Docs => "Docs",
            Self::Archive => "Archive",
            Self::Privacy => "Privacy",
        }
    }
}

impl FromStr for Profile {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "auto" => Ok(Self::Auto),
            "slack" => Ok(Self::Slack),
            "teams" => Ok(Self::Teams),
            "discord" => Ok(Self::Discord),
            "chatwork" => Ok(Self::Chatwork),
            "line" => Ok(Self::Line),
            "whatsapp" => Ok(Self::WhatsApp),
            "docs" => Ok(Self::Docs),
            "archive" => Ok(Self::Archive),
            "privacy" => Ok(Self::Privacy),
            other => Err(format!("unknown profile: {other}")),
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourcePolicy {
    Keep,
    Trash,
    Ask,
}

impl SourcePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Trash => "trash",
            Self::Ask => "ask",
        }
    }
}

impl FromStr for SourcePolicy {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "keep" => Ok(Self::Keep),
            "trash" => Ok(Self::Trash),
            "ask" => Ok(Self::Ask),
            other => Err(format!("unknown source policy: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaInfo {
    pub path: PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<Duration>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodeJob {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub profile: Profile,
    pub output_size: OutputSize,
    pub source_policy: SourcePolicy,
}
