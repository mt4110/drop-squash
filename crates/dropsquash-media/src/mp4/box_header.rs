use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub(super) struct BoxHeader {
    pub(super) kind: [u8; 4],
    pub(super) content_start: u64,
    pub(super) end: u64,
}

pub(super) fn read_box_header(
    file: &mut File,
    offset: u64,
    limit: u64,
) -> std::io::Result<Option<BoxHeader>> {
    let mut header = [0u8; 8];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut header)?;
    let size = u32::from_be_bytes(header[0..4].try_into().unwrap()) as u64;
    let mut kind = [0u8; 4];
    kind.copy_from_slice(&header[4..8]);
    let (content_start, end) = box_bounds(file, offset, limit, size)?;
    if end <= content_start || end > limit {
        return Ok(None);
    }
    Ok(Some(BoxHeader {
        kind,
        content_start,
        end,
    }))
}

fn box_bounds(file: &mut File, offset: u64, limit: u64, size: u64) -> std::io::Result<(u64, u64)> {
    match size {
        0 => Ok((offset + 8, limit)),
        1 => extended_box_bounds(file, offset, limit),
        2..=7 => Ok((offset, offset)),
        _ => Ok((offset + 8, offset.saturating_add(size))),
    }
}

fn extended_box_bounds(file: &mut File, offset: u64, limit: u64) -> std::io::Result<(u64, u64)> {
    let mut size = [0u8; 8];
    file.read_exact(&mut size)?;
    let end = offset.saturating_add(u64::from_be_bytes(size));
    Ok((offset + 16, end.min(limit)))
}
