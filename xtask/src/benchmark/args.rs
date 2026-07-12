use std::path::PathBuf;

use dropsquash_core::{OutputSize, Profile};

mod output_dir_policy;
mod parser;
mod usage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkArgs {
    pub inputs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub csv_output: Option<PathBuf>,
    pub profile: Profile,
    pub release_set: bool,
    pub output_size: OutputSize,
}

impl BenchmarkArgs {
    pub fn parse(args: Vec<String>) -> Result<Self, String> {
        parser::parse(args)
    }
}

#[cfg(test)]
mod tests;
