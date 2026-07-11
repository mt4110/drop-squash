use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{
    default_history_path, default_license_cache_path, AppError, EncodeJob, LicenseState,
    SourcePolicy, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_encoder::EncoderBackend;
use dropsquash_history::{append_record, read_records, ConversionRecord, HistoryMetrics};
use dropsquash_license::{LicenseCache, LicenseGate};

use crate::args::{Cli, Command, OutputSizeArg, ProfileArg};

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!("DropSquash supports macOS, Windows, and Linux");

pub async fn run(cli: Cli) -> dropsquash_core::Result<()> {
    match cli.command {
        Command::Convert {
            input,
            output_dir,
            profile,
            output_size,
            history,
        } => convert(input, output_dir, profile, output_size, history).await,
        Command::Stats { history } => stats(history).await,
        Command::Doctor => doctor(),
    }
}

async fn convert(
    input: PathBuf,
    output_dir: PathBuf,
    profile: ProfileArg,
    output_size: OutputSizeArg,
    history: Option<PathBuf>,
) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    ensure_trial_available(&history).await?;
    let result = NativeEncoder
        .encode(EncodeJob {
            input_path: input,
            output_dir,
            profile: profile.into(),
            output_size: output_size.into(),
            source_policy: SourcePolicy::Ask,
        })
        .await?;
    append_record(&history, &ConversionRecord::new(result.clone())).await?;
    println!("output: {}", result.output_path.display());
    println!("saved bytes: {}", result.saved_bytes());
    println!("reduction: {:.2}%", result.reduction_percent());
    Ok(())
}

async fn stats(history: Option<PathBuf>) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    let records = read_records(&history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    let state = license_gate()?.state_for_metrics(metrics);
    println!(
        "successful conversions: {}",
        metrics.successful_conversion_count
    );
    println!("total original bytes: {}", metrics.total_original_bytes);
    println!("total output bytes: {}", metrics.total_output_bytes);
    println!("saved bytes: {}", metrics.saved_bytes);
    println!(
        "average reduction: {:.2}%",
        metrics.average_reduction_percent
    );
    println!("license state: {state:?}");
    Ok(())
}

fn doctor() -> dropsquash_core::Result<()> {
    let capabilities = NativeEncoder.probe_capabilities()?;
    println!("DropSquash doctor");
    println!("native backend: {}", capabilities.backend_name);
    println!("backend available: {}", capabilities.available);
    println!(
        "hardware acceleration: {}",
        capabilities.hardware_acceleration
    );
    println!("external media executables: disabled");
    println!("payment integration: intentionally absent");
    println!("cloud upload: intentionally absent");
    Ok(())
}

async fn ensure_trial_available(history: &std::path::Path) -> dropsquash_core::Result<()> {
    let records = read_records(history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    if matches!(
        license_gate()?.state_for_metrics(metrics),
        LicenseState::Locked(_)
    ) {
        return Err(AppError::License(
            "trial limit reached; enter a license key to continue".to_string(),
        ));
    }
    Ok(())
}

fn license_gate() -> dropsquash_core::Result<LicenseGate> {
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    Ok(LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: cache.permits_pro(now_unix()),
    })
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
