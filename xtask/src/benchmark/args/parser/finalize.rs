use std::path::PathBuf;

use dropsquash_core::{OutputSize, Profile};

use crate::benchmark::args::{output_dir_policy, usage, BenchmarkArgs};

pub(super) fn finish(
    inputs: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
    profile: Profile,
    release_set: bool,
    output_size: OutputSize,
) -> Result<BenchmarkArgs, String> {
    require_inputs(&inputs, release_set)?;
    let output_dir =
        output_dir.ok_or_else(|| format!("benchmark requires --output-dir\n{}", usage::text()))?;
    output_dir_policy::validate(release_set, &output_dir)?;
    Ok(BenchmarkArgs {
        inputs,
        output_dir,
        profile,
        release_set,
        output_size,
    })
}

fn require_inputs(inputs: &[PathBuf], release_set: bool) -> Result<(), String> {
    if inputs.is_empty() {
        return Err(format!(
            "benchmark requires at least one --input\n{}",
            usage::text()
        ));
    }
    if release_set && inputs.len() < 3 {
        return Err(format!(
            "benchmark --release-set requires at least three --input values\n{}",
            usage::text()
        ));
    }
    Ok(())
}
