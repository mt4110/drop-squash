use std::path::PathBuf;

use dropsquash_fileguard::verify_secure_share_evidence;

pub fn run(video: PathBuf, sidecar: PathBuf) -> dropsquash_core::Result<()> {
    let evidence = verify_secure_share_evidence(&video, &sidecar)?;
    println!("Secure Share evidence verified");
    println!("video: {}", video.display());
    println!("sidecar: {}", sidecar.display());
    println!("masked frames: {}", evidence.plan.frames.len());
    Ok(())
}
