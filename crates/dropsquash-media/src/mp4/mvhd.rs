use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

pub(super) fn read_mvhd_duration(
    file: &mut File,
    content_start: u64,
) -> std::io::Result<Option<Duration>> {
    let mut version = [0u8; 1];
    file.seek(SeekFrom::Start(content_start))?;
    file.read_exact(&mut version)?;
    let (timescale_offset, duration_offset) = if version[0] == 1 { (20, 24) } else { (12, 16) };
    let timescale = read_u32(file, content_start + timescale_offset)?;
    if timescale == 0 {
        return Ok(None);
    }
    let duration = if version[0] == 1 {
        read_u64(file, content_start + duration_offset)?
    } else {
        read_u32(file, content_start + duration_offset)? as u64
    };
    Ok(Some(Duration::from_secs_f64(
        duration as f64 / timescale as f64,
    )))
}

fn read_u32(file: &mut File, offset: u64) -> std::io::Result<u32> {
    let mut value = [0u8; 4];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut value)?;
    Ok(u32::from_be_bytes(value))
}

fn read_u64(file: &mut File, offset: u64) -> std::io::Result<u64> {
    let mut value = [0u8; 8];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut value)?;
    Ok(u64::from_be_bytes(value))
}
