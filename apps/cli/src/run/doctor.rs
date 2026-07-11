use dropsquash_encoder::EncoderBackend;

#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;
#[cfg(target_os = "macos")]
use dropsquash_encoder::VideoToolboxEncoder as NativeEncoder;

pub fn run() -> dropsquash_core::Result<()> {
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
