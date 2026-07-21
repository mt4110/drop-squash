use dropsquash_core::{MaskMode, SecureShareOptions};
use dropsquash_encoder::verify_mask_plan_output;
use dropsquash_fileguard::verify_secure_share_evidence;

const USAGE: &str =
    "usage: cargo run -p xtask -- secure-share-evidence-check <video.mp4> <sidecar.json>";

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    let (video, sidecar) = paths(args)?;
    let evidence =
        verify_secure_share_evidence(&video, &sidecar).map_err(|error| error.to_string())?;
    verify_mask_plan_output(
        &video,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: Vec::new(),
            mask_plan: Some(evidence.plan.clone()),
        },
    )
    .map_err(|error| error.to_string())?;
    println!("Secure Share evidence verified");
    println!("decoded output: verified");
    println!("evidence schema: {}", evidence.schema_version);
    println!("plan schema: {}", evidence.plan.schema_version);
    println!("policy: {:?}", evidence.plan.policy);
    println!(
        "capture continuity attested: {}",
        evidence.plan.audit.capture_continuity_attested == Some(true)
    );
    println!(
        "capture continuity watches: {}",
        evidence.plan.audit.capture_continuity_watches.join(",")
    );
    println!("exposure coverage:");
    for coverage in &evidence.plan.audit.exposure_coverage {
        println!(
            "- {}: {:?} ({})",
            coverage.path, coverage.status, coverage.mitigation
        );
    }
    println!("frames: {}", evidence.plan.frames.len());
    println!("sha256: {}", evidence.output_sha256);
    Ok(())
}

fn paths(args: Vec<String>) -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
    match args.as_slice() {
        [video, sidecar] => Ok((video.into(), sidecar.into())),
        _ => Err(USAGE.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::paths;

    #[test]
    fn requires_video_and_sidecar_paths() {
        assert!(paths(vec!["video.mp4".into()]).is_err());
        assert!(paths(vec!["video.mp4".into(), "sidecar.json".into()]).is_ok());
    }
}
