use std::path::Path;

use dropsquash_core::{AppError, Result, SecureShareOptions};
use dropsquash_media::has_untrusted_metadata_boxes;
use objc2_av_foundation::{
    AVAssetTrack, AVMediaCharacteristicAudible, AVMediaCharacteristicVisual, AVURLAsset,
};

pub(super) fn verify_asset_constraints(
    path: &Path,
    asset: &AVURLAsset,
    options: &SecureShareOptions,
) -> Result<()> {
    let Some(expectations) = options
        .mask_plan
        .as_ref()
        .map(|plan| &plan.verification_expectations)
    else {
        return Ok(());
    };
    let tracks = unsafe { asset.tracks() }.to_vec();
    super::super::secure_share_track_policy::verify(
        tracks.len(),
        tracks.iter().any(|track| !is_visual(track)),
    )?;
    if expectations.no_audio && tracks.iter().any(|track| has_audio(track)) {
        return Err(failed("found an audio track"));
    }
    if expectations.strip_metadata && metadata_count(asset, &tracks) != 0 {
        return Err(failed("found output metadata"));
    }
    if expectations.strip_metadata && has_untrusted_metadata_boxes(path)? {
        return Err(failed("found an untrusted MP4 metadata box"));
    }
    Ok(())
}

fn is_visual(track: &AVAssetTrack) -> bool {
    unsafe { track.hasMediaCharacteristic(AVMediaCharacteristicVisual.unwrap()) }
}

fn has_audio(track: &AVAssetTrack) -> bool {
    unsafe { track.hasMediaCharacteristic(AVMediaCharacteristicAudible.unwrap()) }
}

fn metadata_count(asset: &AVURLAsset, tracks: &[objc2::rc::Retained<AVAssetTrack>]) -> usize {
    unsafe { asset.metadata() }.count()
        + tracks
            .iter()
            .map(|track| unsafe { track.metadata() }.count())
            .sum::<usize>()
}

fn failed(reason: &str) -> AppError {
    AppError::Encoder(format!("Secure Share verification {reason}"))
}
