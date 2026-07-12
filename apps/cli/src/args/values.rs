use clap::ValueEnum;
use dropsquash_core::{OutputSize, Profile};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ProfileArg {
    Auto,
    Slack,
    Docs,
    Teams,
    Discord,
    Chatwork,
    Line,
    #[value(name = "whatsapp")]
    WhatsApp,
    Archive,
    Privacy,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputSizeArg {
    Auto,
    #[value(name = "1080p")]
    P1080,
    #[value(name = "720p")]
    P720,
    #[value(name = "480p")]
    P480,
}

impl From<ProfileArg> for Profile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::Auto => Self::Auto,
            ProfileArg::Slack => Self::Slack,
            ProfileArg::Docs => Self::Docs,
            ProfileArg::Teams => Self::Teams,
            ProfileArg::Discord => Self::Discord,
            ProfileArg::Chatwork => Self::Chatwork,
            ProfileArg::Line => Self::Line,
            ProfileArg::WhatsApp => Self::WhatsApp,
            ProfileArg::Archive => Self::Archive,
            ProfileArg::Privacy => Self::Privacy,
        }
    }
}

impl From<OutputSizeArg> for OutputSize {
    fn from(value: OutputSizeArg) -> Self {
        match value {
            OutputSizeArg::Auto => Self::Auto,
            OutputSizeArg::P1080 => Self::P1080,
            OutputSizeArg::P720 => Self::P720,
            OutputSizeArg::P480 => Self::P480,
        }
    }
}
