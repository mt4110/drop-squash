use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use dropsquash_core::{
    default_history_path, EncodeJob, OutputSize, Profile, SourcePolicy, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_encoder::EncoderBackend;
use dropsquash_history::{append_record, read_records, ConversionRecord, HistoryMetrics};
use dropsquash_license::LicenseGate;

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!("DropSquash supports macOS, Windows, and Linux");

#[derive(Debug, Parser)]
#[command(name = "dropsquash")]
#[command(about = "Drop huge screen recordings. Squash them locally.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Convert {
        input: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
        #[arg(long, value_enum, default_value_t = ProfileArg::Auto)]
        profile: ProfileArg,
        #[arg(long, value_enum, default_value_t = OutputSizeArg::Auto)]
        output_size: OutputSizeArg,
        #[arg(long)]
        history: Option<PathBuf>,
    },
    Stats {
        #[arg(long)]
        history: Option<PathBuf>,
    },
    Doctor,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ProfileArg {
    Auto,
    Slack,
    Docs,
    Teams,
    Discord,
    Chatwork,
    Line,
    #[value(name = "whatsapp")]
    WhatsApp,
    Archive,
    Privacy,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputSizeArg {
    Auto,
    #[value(name = "1080p")]
    P1080,
    #[value(name = "720p")]
    P720,
    #[value(name = "480p")]
    P480,
}

impl From<ProfileArg> for Profile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::Auto => Self::Auto,
            ProfileArg::Slack => Self::Slack,
            ProfileArg::Docs => Self::Docs,
            ProfileArg::Teams => Self::Teams,
            ProfileArg::Discord => Self::Discord,
            ProfileArg::Chatwork => Self::Chatwork,
            ProfileArg::Line => Self::Line,
            ProfileArg::WhatsApp => Self::WhatsApp,
            ProfileArg::Archive => Self::Archive,
            ProfileArg::Privacy => Self::Privacy,
        }
    }
}

impl From<OutputSizeArg> for OutputSize {
    fn from(value: OutputSizeArg) -> Self {
        match value {
            OutputSizeArg::Auto => Self::Auto,
            OutputSizeArg::P1080 => Self::P1080,
            OutputSizeArg::P720 => Self::P720,
            OutputSizeArg::P480 => Self::P480,
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match run(Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> dropsquash_core::Result<()> {
    match cli.command {
        Command::Convert {
            input,
            output_dir,
            profile,
            output_size,
            history,
        } => {
            let history = history.unwrap_or_else(default_history_path);
            let records = read_records(&history).await?;
            let metrics = HistoryMetrics::from_records(&records);
            let license_gate = LicenseGate {
                trial_limit: TRIAL_CONVERSION_LIMIT,
                has_valid_license: false,
            };
            if matches!(
                license_gate.state_for_metrics(metrics),
                dropsquash_core::LicenseState::Locked(_)
            ) {
                return Err(dropsquash_core::AppError::License(
                    "trial limit reached; enter a license key to continue".to_string(),
                ));
            }
            let job = EncodeJob {
                input_path: input,
                output_dir,
                profile: profile.into(),
                output_size: output_size.into(),
                source_policy: SourcePolicy::Ask,
            };
            let result = NativeEncoder.encode(job).await?;
            append_record(&history, &ConversionRecord::new(result.clone())).await?;
            println!("output: {}", result.output_path.display());
            println!("saved bytes: {}", result.saved_bytes());
            println!("reduction: {:.2}%", result.reduction_percent());
        }
        Command::Stats { history } => {
            let history = history.unwrap_or_else(default_history_path);
            let records = read_records(&history).await?;
            let metrics = HistoryMetrics::from_records(&records);
            let state = LicenseGate {
                trial_limit: TRIAL_CONVERSION_LIMIT,
                has_valid_license: false,
            }
            .state_for_metrics(metrics);
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
        }
        Command::Doctor => {
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
        }
    }

    Ok(())
}
