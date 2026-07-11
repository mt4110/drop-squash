pub(super) const DISALLOWED: &[&str] = &[
    "@ffmpeg/",
    "avconv",
    "child_process",
    "command::new",
    "ffmpeg",
    "ffprobe",
    "fluent-ffmpeg",
    "\"mediainfo\"",
    "'mediainfo'",
    "mediainfo.js",
    "std::process::command",
    "tokio::process",
];
