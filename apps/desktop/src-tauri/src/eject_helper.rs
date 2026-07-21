use std::{path::Path, thread, time::Duration};

const FLAG: &str = "--dropsquash-eject-volume";
const PID_FLAG: &str = "--dropsquash-wait-pid";

pub fn run_from_args(args: &[String]) -> Option<Result<(), String>> {
    let request = parse(args)?;
    Some(run(request))
}

fn run(request: Request) -> Result<(), String> {
    wait_for_exit(request.pid);
    dropsquash_platform::eject_mounted_volume(Path::new(&request.volume)).map_err(|error| {
        format!(
            "failed to eject installer volume {}: {error}",
            request.volume
        )
    })
}

fn wait_for_exit(pid: u32) {
    for _ in 0..80 {
        if parent_exited(pid) {
            return;
        }
        thread::sleep(Duration::from_millis(250));
    }
}

fn parent_exited(pid: u32) -> bool {
    parent_exited_impl(pid)
}

#[cfg(unix)]
fn parent_exited_impl(pid: u32) -> bool {
    let missing = unsafe { libc::kill(pid as i32, 0) != 0 };
    missing && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}

#[cfg(not(unix))]
fn parent_exited_impl(_pid: u32) -> bool {
    true
}

fn parse(args: &[String]) -> Option<Request> {
    let volume = flag_value(args, FLAG)?;
    let pid = flag_value(args, PID_FLAG)?.parse().ok()?;
    Some(Request { volume, pid })
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].clone())
}

struct Request {
    volume: String,
    pid: u32,
}

#[cfg(test)]
mod tests {
    use super::run_from_args;

    #[test]
    fn ignores_normal_app_args() {
        assert!(run_from_args(&["dropsquash-desktop".into()]).is_none());
    }

    #[test]
    fn parses_helper_request() {
        let result = run_from_args(&[
            "dropsquash-desktop".into(),
            "--dropsquash-eject-volume".into(),
            "/Volumes/DropSquash".into(),
            "--dropsquash-wait-pid".into(),
            "42".into(),
        ]);
        assert!(result.is_some());
    }
}
