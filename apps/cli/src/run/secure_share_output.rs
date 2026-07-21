use std::path::PathBuf;

use dropsquash_core::{MaskMode, SecureShareOptions};
use dropsquash_encoder::verify_mask_plan_output;
use dropsquash_fileguard::verify_secure_share_evidence;

pub fn run(video: PathBuf, sidecar: PathBuf) -> dropsquash_core::Result<()> {
    let evidence = verify_secure_share_evidence(&video, &sidecar)?;
    verify_mask_plan_output(
        &video,
        &SecureShareOptions {
            mask_mode: MaskMode::SolidBlack,
            mask_rects: Vec::new(),
            mask_plan: Some(evidence.plan),
        },
    )?;
    println!("Secure Share output independently verified");
    println!("video: {}", video.display());
    println!("sidecar: {}", sidecar.display());
    Ok(())
}
