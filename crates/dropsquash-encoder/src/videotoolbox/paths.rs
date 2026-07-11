use dropsquash_core::{EncodeJob, Result};
use std::path::PathBuf;

pub(crate) fn output_path_for(job: &EncodeJob) -> Result<PathBuf> {
    let stem = job
        .input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recording");
    let primary = job.output_dir.join(format!("{stem}.squashed.mp4"));
    if !primary.try_exists()? {
        return Ok(primary);
    }

    for suffix in 2u32.. {
        let candidate = job.output_dir.join(format!("{stem}.squashed-{suffix}.mp4"));
        if !candidate.try_exists()? {
            return Ok(candidate);
        }
    }

    unreachable!("unbounded output suffix range must contain an available path")
}
