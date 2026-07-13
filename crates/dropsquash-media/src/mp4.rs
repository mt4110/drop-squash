mod box_header;
mod mvhd;

use std::fs::File;
use std::path::Path;
use std::time::Duration;

use box_header::read_box_header;
use mvhd::read_mvhd_duration;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Mp4Inspection {
    pub has_file_type: bool,
    pub duration: Option<Duration>,
}

impl Mp4Inspection {
    pub fn has_nonzero_duration(&self) -> bool {
        self.duration.is_some_and(|duration| !duration.is_zero())
    }
}

pub fn inspect_mp4(path: &Path) -> Mp4Inspection {
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
            inspection = inspection.merge(inspect_range(file, header.content_start, header.end)?);
        } else if &header.kind == b"mvhd" {
            inspection.duration = read_mvhd_duration(file, header.content_start)?;
        }
        if inspection.has_file_type && inspection.has_nonzero_duration() {
            break;
        }
        cursor = header.end;
    }
    Ok(inspection)
}

impl Mp4Inspection {
    fn merge(self, child: Self) -> Self {
        Self {
            has_file_type: self.has_file_type || child.has_file_type,
            duration: self.duration.or(child.duration),
        }
    }
}

#[cfg(test)]
mod tests;
