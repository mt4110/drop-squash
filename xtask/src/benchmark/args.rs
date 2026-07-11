use std::path::PathBuf;
use std::str::FromStr;

use dropsquash_core::{OutputSize, Profile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkArgs {
    pub inputs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub profile: Profile,
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
    output_size: OutputSize,
}

impl Parser {
    fn new(args: Vec<String>) -> Self {
        Self {
            args: args.into_iter(),
            inputs: Vec::new(),
            output_dir: None,
            profile: Profile::Auto,
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
        Ok(BenchmarkArgs {
            inputs: self.inputs,
            output_dir: self
                .output_dir
                .ok_or_else(|| format!("benchmark requires --output-dir\n{}", usage()))?,
            profile: self.profile,
            output_size: self.output_size,
        })
    }
}

fn usage() -> String {
    "usage: cargo run -p xtask -- benchmark --input <movie> --output-dir <dir> [--profile auto] [--size auto]".to_string()
}

#[cfg(test)]
mod tests;
