mod args;
mod report;
mod validate;

use std::fs;
use std::time::Instant;

use dropsquash_core::{EncodeJob, SourcePolicy};
use dropsquash_encoder::EncoderBackend;

use args::BenchmarkArgs;
use report::BenchmarkRow;

#[cfg(target_os = "macos")]
use dropsquash_encoder::AppleNativeEncoder as NativeEncoder;
#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let args = BenchmarkArgs::parse(args)?;
    fs::create_dir_all(&args.output_dir).map_err(|error| error.to_string())?;
    let runtime = tokio::runtime::Runtime::new().map_err(|error| error.to_string())?;
    runtime.block_on(run_async(args))
}

async fn run_async(args: BenchmarkArgs) -> Result<(), String> {
    let encoder = NativeEncoder;
    let capabilities = encoder
        .probe_capabilities()
        .map_err(|error| error.to_string())?;
    if !capabilities.available {
        return Err(format!("{} backend is unavailable", encoder.name()));
    }

    let mut rows = Vec::new();
    for input_path in args.inputs {
        let started = Instant::now();
        let result = encoder
            .encode(EncodeJob {
                input_path,
                output_dir: args.output_dir.clone(),
                profile: args.profile,
                output_size: args.output_size,
                source_policy: SourcePolicy::Keep,
            })
            .await
            .map_err(|error| error.to_string())?;
        validate::result(&result)?;
        rows.push(BenchmarkRow::from_result(
            encoder.name(),
            result,
            started.elapsed(),
        ));
    }
    if let Some(path) = args.csv_output {
        report::write(&path, &rows)?;
    } else {
        report::print(&rows);
    }
    Ok(())
}
