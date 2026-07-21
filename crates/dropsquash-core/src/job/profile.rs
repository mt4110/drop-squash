use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

    pub const DESKTOP: [Self; 4] = [Self::Auto, Self::Slack, Self::Docs, Self::Archive];

    pub fn desktop_profile(self) -> Self {
        match self {
            Self::Teams | Self::Discord | Self::Chatwork | Self::Line | Self::WhatsApp => {
                Self::Slack
            }
            Self::Privacy => Self::Archive,
            Self::Auto | Self::Slack | Self::Docs | Self::Archive => self,
        }
    }

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
            Self::Auto => "自動 / Auto",
            Self::Slack => "共有 / Slack",
            Self::Teams => "会議 / Microsoft Teams",
            Self::Discord => "会話 / Discord",
            Self::Chatwork => "業務連絡 / Chatwork",
            Self::Line => "LINE",
            Self::WhatsApp => "WhatsApp",
            Self::Docs => "文書 / Docs",
            Self::Archive => "保管 / Archive",
            Self::Privacy => "秘匿 / Privacy",
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
