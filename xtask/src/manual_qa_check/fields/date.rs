pub(super) fn is_iso(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !has_iso_shape(bytes) {
        return false;
    }
    let year = value[..4].parse::<u16>().unwrap_or(0);
    let month = value[5..7].parse::<u8>().unwrap_or(0);
    let day = value[8..].parse::<u8>().unwrap_or(0);
    year >= 2000 && (1..=days_in_month(year, month)).contains(&day)
}

fn has_iso_shape(bytes: &[u8]) -> bool {
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[8..].iter().all(u8::is_ascii_digit)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
