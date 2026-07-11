use dropsquash_core::{AppError, Result};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub(crate) fn finalize_verified_output(temporary_output: &Path, output_path: &Path) -> Result<()> {
    if !temporary_output.is_file() {
        return Err(AppError::FileNotFound(temporary_output.to_path_buf()));
    }
    if output_path.try_exists()? {
        return Err(AppError::Encoder(format!(
            "output already exists: {}",
            output_path.display()
        )));
    }

    let temporary_output = path_to_c_string(temporary_output)?;
    let output_path = path_to_c_string(output_path)?;
    let result = unsafe {
        libc::renamex_np(
            temporary_output.as_ptr(),
            output_path.as_ptr(),
            libc::RENAME_EXCL,
        )
    };
    if result == 0 {
        return Ok(());
    }
    Err(AppError::Io(std::io::Error::last_os_error()))
}

fn path_to_c_string(path: &Path) -> Result<CString> {
    CString::new(path.as_os_str().as_bytes()).map_err(|_| {
        AppError::Encoder(format!(
            "path contains an interior NUL byte: {}",
            path.display()
        ))
    })
}
