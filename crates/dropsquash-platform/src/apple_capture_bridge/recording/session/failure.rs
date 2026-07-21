#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeRecordingFailure {
    Authorization,
    Target,
    Setup,
    Environment,
    Writer,
    Continuity,
    Unknown,
}

impl NativeRecordingFailure {
    pub fn user_message(self) -> &'static str {
        match self {
            Self::Authorization => "Screen Recording or Accessibility permission is required",
            Self::Target => "The selected window changed or could not be verified",
            Self::Setup => "Secure Share capture could not be initialized",
            Self::Environment => "The display, session, or active app changed during recording",
            Self::Writer => "Secure Share video writing could not be verified",
            Self::Continuity => "Secure Share frame continuity could not be verified",
            Self::Unknown => "Secure Share native recording failed closed",
        }
    }
}

impl From<i32> for NativeRecordingFailure {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Authorization,
            2 => Self::Target,
            3 => Self::Setup,
            4 => Self::Environment,
            5 => Self::Writer,
            6 => Self::Continuity,
            _ => Self::Unknown,
        }
    }
}
