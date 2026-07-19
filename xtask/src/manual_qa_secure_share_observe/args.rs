pub(super) const USAGE: &str =
    "usage: cargo run -p xtask -- manual-qa-secure-share-observe [list|auto|window <id>] [capture-ms] [timeout-ms]";
const DEFAULT_CAPTURE_MS: u64 = 150;
const DEFAULT_TIMEOUT_MS: u64 = 5_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Options {
    pub mode: Mode,
    pub capture_ms: u64,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Mode {
    List,
    Auto,
    Window(u32),
}

pub(super) fn parse_args(args: Vec<String>) -> Result<Options, String> {
    match args.as_slice() {
        [] => Ok(options(Mode::Auto, DEFAULT_CAPTURE_MS, DEFAULT_TIMEOUT_MS)),
        [mode] if mode == "list" => Ok(options(Mode::List, DEFAULT_CAPTURE_MS, DEFAULT_TIMEOUT_MS)),
        [mode] if mode == "auto" => Ok(options(Mode::Auto, DEFAULT_CAPTURE_MS, DEFAULT_TIMEOUT_MS)),
        [capture] => Ok(options(
            Mode::Auto,
            parse_ms(capture, "capture-ms")?,
            DEFAULT_TIMEOUT_MS,
        )),
        [capture, timeout] if capture != "window" => Ok(options(
            Mode::Auto,
            parse_ms(capture, "capture-ms")?,
            parse_ms(timeout, "timeout-ms")?,
        )),
        [mode, id] if mode == "window" => Ok(options(
            Mode::Window(parse_id(id)?),
            DEFAULT_CAPTURE_MS,
            DEFAULT_TIMEOUT_MS,
        )),
        [mode, id, capture] if mode == "window" => Ok(options(
            Mode::Window(parse_id(id)?),
            parse_ms(capture, "capture-ms")?,
            DEFAULT_TIMEOUT_MS,
        )),
        [mode, id, capture, timeout] if mode == "window" => Ok(options(
            Mode::Window(parse_id(id)?),
            parse_ms(capture, "capture-ms")?,
            parse_ms(timeout, "timeout-ms")?,
        )),
        _ => Err(USAGE.to_string()),
    }
}

fn options(mode: Mode, capture_ms: u64, timeout_ms: u64) -> Options {
    Options {
        mode,
        capture_ms,
        timeout_ms,
    }
}

fn parse_id(value: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|error| format!("invalid window id: {error}"))
}

fn parse_ms(value: &str, name: &str) -> Result<u64, String> {
    let millis = value
        .parse::<u64>()
        .map_err(|error| format!("invalid {name}: {error}"))?;
    if millis == 0 {
        return Err(format!("{name} must be greater than 0"));
    }
    Ok(millis)
}
