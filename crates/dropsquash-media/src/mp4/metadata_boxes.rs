use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use super::box_header::read_box_header;

pub fn has_untrusted_metadata_boxes(path: &Path) -> Result<bool> {
    let mut file = File::open(path)?;
    let end = file.metadata()?.len();
    scan(&mut file, 0, end, false)
}

fn scan(file: &mut File, start: u64, end: u64, in_user_data: bool) -> Result<bool> {
    let mut cursor = start;
    while cursor + 8 <= end {
        let header = read_box_header(file, cursor, end)?.ok_or_else(invalid_box)?;
        if header.end <= cursor {
            return Err(invalid_box());
        }
        if forbidden(header.kind, in_user_data) {
            return Ok(true);
        }
        if let Some(child_start) = child_start(header.kind, header.content_start, header.end) {
            if scan(file, child_start, header.end, header.kind == *b"udta")? {
                return Ok(true);
            }
        }
        cursor = header.end;
    }
    (cursor == end).then_some(false).ok_or_else(invalid_box)
}

fn forbidden(kind: [u8; 4], in_user_data: bool) -> bool {
    in_user_data && kind != *b"meta"
        || matches!(
            &kind,
            b"uuid"
                | b"emsg"
                | b"ilst"
                | b"keys"
                | b"xml "
                | b"bxml"
                | b"cprt"
                | b"loci"
                | b"kind"
                | b"name"
                | b"auth"
                | b"titl"
                | b"desc"
                | b"perf"
        )
        || kind[0] == 0xa9
}

fn child_start(kind: [u8; 4], start: u64, end: u64) -> Option<u64> {
    if kind == *b"meta" {
        return start
            .checked_add(4)
            .filter(|child_start| *child_start <= end);
    }
    matches!(
        &kind,
        b"moov"
            | b"trak"
            | b"mdia"
            | b"minf"
            | b"stbl"
            | b"edts"
            | b"dinf"
            | b"mvex"
            | b"moof"
            | b"traf"
            | b"udta"
    )
    .then_some(start)
}

fn invalid_box() -> Error {
    Error::new(ErrorKind::InvalidData, "invalid MP4 box structure")
}

#[cfg(test)]
mod tests {
    use super::has_untrusted_metadata_boxes;

    #[test]
    fn accepts_an_empty_standard_metadata_container() {
        let (_directory, path) = fixture(&atom(*b"meta", &[0, 0, 0, 0]));
        assert!(!has_untrusted_metadata_boxes(&path).unwrap());
    }

    #[test]
    fn detects_user_data_payloads_and_timed_events() {
        for payload in [
            atom(*b"udta", &atom(*b"name", b"secret")),
            atom(*b"emsg", b"secret"),
            atom(*b"xml ", b"<secret>value</secret>"),
            atom(*b"cprt", b"private owner"),
        ] {
            let (_directory, path) = fixture(&payload);
            assert!(has_untrusted_metadata_boxes(&path).unwrap());
        }
    }

    #[test]
    fn rejects_an_invalid_box_structure() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("invalid.mp4");
        std::fs::write(&path, [0, 0, 0, 2, b'f', b'r', b'e', b'e']).unwrap();
        assert!(has_untrusted_metadata_boxes(&path).is_err());
    }

    fn fixture(payload: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("fixture.mp4");
        std::fs::write(&path, payload).unwrap();
        (directory, path)
    }

    fn atom(kind: [u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut bytes = ((payload.len() + 8) as u32).to_be_bytes().to_vec();
        bytes.extend(kind);
        bytes.extend(payload);
        bytes
    }
}
