#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopEvent {
    FileDropped(String),
    ConversionQueued(String),
    ConversionFinished(String),
}
