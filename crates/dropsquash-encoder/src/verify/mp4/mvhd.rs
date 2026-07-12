use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub(super) fn read_mvhd_duration(file: &mut File, content_start: u64) -> std::io::Result<u64> {
    let mut version = [0u8; 1];
    file.seek(SeekFrom::Start(content_start))?;
    file.read_exact(&mut version)?;
    let duration_offset = if version[0] == 1 { 28 } else { 16 };
    file.seek(SeekFrom::Start(content_start + duration_offset))?;
    if version[0] == 1 {
        read_duration_v1(file)
    } else {
        read_duration_v0(file)
    }
}

fn read_duration_v1(file: &mut File) -> std::io::Result<u64> {
    let mut duration = [0u8; 8];
    file.read_exact(&mut duration)?;
    Ok(u64::from_be_bytes(duration))
}

fn read_duration_v0(file: &mut File) -> std::io::Result<u64> {
    let mut duration = [0u8; 4];
    file.read_exact(&mut duration)?;
    Ok(u32::from_be_bytes(duration) as u64)
}
