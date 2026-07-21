use dropsquash_core::{AppError, Result};

#[repr(C)]
struct MachTimebaseInfo {
    numer: u32,
    denom: u32,
}

unsafe extern "C" {
    fn mach_timebase_info(info: *mut MachTimebaseInfo) -> i32;
}

pub(super) fn nanoseconds(ticks: u64) -> Result<u64> {
    let mut info = MachTimebaseInfo { numer: 0, denom: 0 };
    let status = unsafe { mach_timebase_info(&mut info) };
    if status != 0 || info.denom == 0 {
        return Err(AppError::InvalidConfig(
            "Secure Share could not read the macOS monotonic clock timebase".to_string(),
        ));
    }
    scale(ticks, u64::from(info.numer), u64::from(info.denom))
}

fn scale(ticks: u64, numer: u64, denom: u64) -> Result<u64> {
    if denom == 0 {
        return Err(AppError::InvalidConfig(
            "Secure Share macOS monotonic clock timebase has zero denominator".to_string(),
        ));
    }
    u64::try_from(u128::from(ticks) * u128::from(numer) / u128::from(denom)).map_err(|_| {
        AppError::InvalidConfig("Secure Share monotonic time overflows nanoseconds".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::scale;

    #[test]
    fn converts_mach_ticks_without_losing_precision() {
        assert_eq!(scale(3, 125, 3).unwrap(), 125);
    }
}
