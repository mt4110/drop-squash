use super::USAGE;

pub(super) type Parsed = (
    bool,
    bool,
    bool,
    Option<String>,
    u64,
    Option<String>,
    String,
    String,
);

pub(super) fn parse_args(args: Vec<String>) -> Result<Parsed, String> {
    let mut event_log = false;
    let mut mount_dmg = false;
    let mut open_panel = false;
    let mut open_file = None;
    let mut settle_seconds = 5;
    let mut license_api_base_url = None;
    let mut positionals = Vec::new();
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--event-log" => event_log = true,
            "--mount-dmg" => mount_dmg = true,
            "--open-panel" => open_panel = true,
            "--open-file" => open_file = Some(iter.next().ok_or_else(|| USAGE.to_string())?),
            "--settle-seconds" => {
                settle_seconds = iter
                    .next()
                    .ok_or_else(|| USAGE.to_string())?
                    .parse()
                    .map_err(|_| USAGE.to_string())?
            }
            "--license-api-base-url" => {
                license_api_base_url = Some(iter.next().ok_or_else(|| USAGE.to_string())?)
            }
            _ => positionals.push(arg),
        }
    }
    match positionals.as_slice() {
        [artifact, config] => Ok((
            event_log,
            mount_dmg,
            open_panel,
            open_file,
            settle_seconds,
            license_api_base_url,
            artifact.clone(),
            config.clone(),
        )),
        _ => Err(USAGE.to_string()),
    }
}
