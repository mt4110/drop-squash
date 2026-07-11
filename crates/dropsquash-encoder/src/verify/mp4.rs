use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, Copy, Default)]
pub struct Mp4Inspection {
    pub has_file_type: bool,
    pub has_nonzero_duration: bool,
}

pub fn inspect(path: &Path) -> Mp4Inspection {
    let Ok(mut file) = File::open(path) else {
        return Mp4Inspection::default();
    };
    let Ok(file_len) = file.metadata().map(|metadata| metadata.len()) else {
        return Mp4Inspection::default();
    };
    inspect_range(&mut file, 0, file_len).unwrap_or_default()
}

fn inspect_range(file: &mut File, start: u64, end: u64) -> std::io::Result<Mp4Inspection> {
    let mut cursor = start;
    let mut inspection = Mp4Inspection::default();
    while cursor + 8 <= end {
        let Some(header) = read_box_header(file, cursor, end)? else {
            break;
        };
        inspection.has_file_type |= &header.kind == b"ftyp";
        if &header.kind == b"moov" {
            let child = inspect_range(file, header.content_start, header.end)?;
            inspection.has_nonzero_duration |= child.has_nonzero_duration;
        } else if &header.kind == b"mvhd" {
            inspection.has_nonzero_duration |= read_mvhd_duration(file, header.content_start)? > 0;
        }
        if inspection.has_file_type && inspection.has_nonzero_duration {
            break;
        }
        cursor = header.end;
    }
    Ok(inspection)
}

struct BoxHeader {
    kind: [u8; 4],
    content_start: u64,
    end: u64,
}

fn read_box_header(file: &mut File, offset: u64, limit: u64) -> std::io::Result<Option<BoxHeader>> {
    let mut header = [0u8; 8];
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(&mut header)?;
    let size = u32::from_be_bytes(header[0..4].try_into().unwrap()) as u64;
    let mut kind = [0u8; 4];
    kind.copy_from_slice(&header[4..8]);
    let (content_start, end) = match size {
        0 => (offset + 8, limit),
        1 => extended_box_bounds(file, offset, limit)?,
        2..=7 => return Ok(None),
        _ => (offset + 8, offset.saturating_add(size)),
    };
    if end <= content_start || end > limit {
        return Ok(None);
    }
    Ok(Some(BoxHeader {
        kind,
        content_start,
        end,
    }))
}

fn extended_box_bounds(file: &mut File, offset: u64, limit: u64) -> std::io::Result<(u64, u64)> {
    let mut size = [0u8; 8];
    file.read_exact(&mut size)?;
    let end = offset.saturating_add(u64::from_be_bytes(size));
    Ok((offset + 16, end.min(limit)))
}

fn read_mvhd_duration(file: &mut File, content_start: u64) -> std::io::Result<u64> {
    let mut version = [0u8; 1];
    file.seek(SeekFrom::Start(content_start))?;
    file.read_exact(&mut version)?;
    let duration_offset = if version[0] == 1 { 28 } else { 16 };
    file.seek(SeekFrom::Start(content_start + duration_offset))?;
    if version[0] == 1 {
        let mut duration = [0u8; 8];
        file.read_exact(&mut duration)?;
        Ok(u64::from_be_bytes(duration))
    } else {
        let mut duration = [0u8; 4];
        file.read_exact(&mut duration)?;
        Ok(u32::from_be_bytes(duration) as u64)
    }
}
