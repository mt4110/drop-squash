use std::path::PathBuf;
use std::str::FromStr;

use dropsquash_core::{OutputSize, Profile};

mod output_dir_policy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkArgs {
    pub inputs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub profile: Profile,
    pub release_set: bool,
    pub output_size: OutputSize,
}

impl BenchmarkArgs {
    pub fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut parser = Parser::new(args);
        while let Some(arg) = parser.next() {
            match arg.as_str() {
                "--input" => parser.input()?,
                "--output-dir" => parser.output_dir()?,
                "--profile" => parser.profile()?,
                "--size" => parser.output_size()?,
                "--release-set" => parser.release_set = true,
                "--help" | "-h" => return Err(usage()),
                other => return Err(format!("unknown benchmark argument: {other}\n{}", usage())),
            }
        }
        parser.finish()
    }
}

struct Parser {
    args: std::vec::IntoIter<String>,
    inputs: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
    profile: Profile,
    release_set: bool,
    output_size: OutputSize,
}

impl Parser {
    fn new(args: Vec<String>) -> Self {
        Self {
            args: args.into_iter(),
            inputs: Vec::new(),
            output_dir: None,
            profile: Profile::Auto,
            release_set: false,
            output_size: OutputSize::Auto,
        }
    }

    fn next(&mut self) -> Option<String> {
        self.args.next()
    }

    fn value(&mut self, name: &str) -> Result<String, String> {
        self.args
            .next()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("{name} requires a value"))
    }

    fn input(&mut self) -> Result<(), String> {
        let input = self.value("--input")?;
        self.inputs.push(PathBuf::from(input));
        Ok(())
    }

    fn output_dir(&mut self) -> Result<(), String> {
        self.output_dir = Some(PathBuf::from(self.value("--output-dir")?));
        Ok(())
    }

    fn profile(&mut self) -> Result<(), String> {
        self.profile = Profile::from_str(&self.value("--profile")?)?;
        Ok(())
    }

    fn output_size(&mut self) -> Result<(), String> {
        self.output_size = OutputSize::from_str(&self.value("--size")?)?;
        Ok(())
    }

    fn finish(self) -> Result<BenchmarkArgs, String> {
        if self.inputs.is_empty() {
            return Err(format!(
                "benchmark requires at least one --input\n{}",
                usage()
            ));
        }
        if self.release_set && self.inputs.len() < 3 {
            return Err(format!(
                "benchmark --release-set requires at least three --input values\n{}",
                usage()
            ));
        }
        let output_dir = self
            .output_dir
            .ok_or_else(|| format!("benchmark requires --output-dir\n{}", usage()))?;
        output_dir_policy::validate(self.release_set, &output_dir)?;
        Ok(BenchmarkArgs {
            inputs: self.inputs,
            output_dir,
            profile: self.profile,
            release_set: self.release_set,
            output_size: self.output_size,
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- benchmark --input <movie> --output-dir <dir> [--profile auto] [--size auto] [--release-set]".to_string()
}

#[cfg(test)]
mod tests;
