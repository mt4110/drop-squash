mod box_header;
mod mvhd;

use box_header::read_box_header;
use mvhd::read_mvhd_duration;
use std::fs::File;
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
