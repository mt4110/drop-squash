use std::path::Path;

use dropsquash_core::{AppError, MaskRect, Result};
use objc2::rc::Retained;
use objc2_av_foundation::{
    AVAssetReader, AVAssetReaderTrackOutput, AVAssetTrack, AVMediaCharacteristicVisual, AVURLAsset,
};

use super::pixel_buffer::{read_sample_buffer_row_signals, RowSignals};
use super::secure_share_settings::reader_output_settings;
use super::session::file_url;

mod bands;

pub(crate) fn detect_auto_mask_rects(input_path: &Path) -> Result<Vec<MaskRect>> {
    let reader = bootstrap_reader(input_path)?;
    if !unsafe { reader.reader.startReading() } {
        return Err(AppError::Encoder(
            "Secure Share auto-detect reader failed to start".to_string(),
        ));
    }
    let mut frame = 0u32;
    let mut compared = 0u32;
    let mut motion = Vec::new();
    let mut edge = Vec::new();
    let mut prev = None;
    let mut width = 0u32;
    let mut height = 0u32;
    while compared < 8 {
        let Some(sample) = (unsafe { reader.output.copyNextSampleBuffer() }) else {
            break;
        };
        if unsafe { sample.num_samples() } == 0 || frame % 6 != 0 {
            frame += 1;
            continue;
        }
        let signal = read_sample_buffer_row_signals(&sample)?;
        width = signal.width;
        height = signal.height;
        seed_scores(&signal, &mut motion, &mut edge);
        if let Some(previous) = prev.replace(signal.brightness.clone()) {
            accumulate_motion(&previous, &signal.brightness, &mut motion);
            compared += 1;
        }
        keep_edge_max(&signal, &mut edge);
        frame += 1;
    }
    let rects = bands::detect_row_rects(width, height, &motion, &edge, compared);
    if rects.is_empty() {
        return Err(AppError::UnsupportedMedia(
            "automatic fixed-bar masking could not find a stable top or bottom bar".to_string(),
        ));
    }
    Ok(rects)
}

fn bootstrap_reader(input_path: &Path) -> Result<ReaderBootstrap> {
    let input_url = file_url(input_path)?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&input_url, None) };
    let visual = unsafe { AVMediaCharacteristicVisual }.unwrap();
    let track = unsafe { asset.tracks() }
        .to_vec()
        .into_iter()
        .find(|track| unsafe { track.hasMediaCharacteristic(visual) })
        .ok_or_else(|| {
            AppError::UnsupportedMedia("Secure Share requires a video track".to_string())
        })?;
    let reader = unsafe { AVAssetReader::assetReaderWithAsset_error(&asset) }
        .map_err(|error| AppError::Encoder(error.localizedDescription().to_string()))?;
    let output = unsafe {
        AVAssetReaderTrackOutput::assetReaderTrackOutputWithTrack_outputSettings(
            &track,
            Some(&reader_output_settings()),
        )
    };
    unsafe { reader.addOutput(&output) };
    Ok(ReaderBootstrap {
        reader,
        output,
        _track: track,
    })
}

fn seed_scores(signal: &RowSignals, motion: &mut Vec<u32>, edge: &mut Vec<u8>) {
    if motion.is_empty() {
        motion.resize(signal.height as usize, 0);
        edge.resize(signal.height as usize, 0);
    }
}

fn accumulate_motion(previous: &[u8], current: &[u8], motion: &mut [u32]) {
    for (index, value) in current.iter().enumerate() {
        motion[index] += value.abs_diff(previous[index]) as u32;
    }
}

fn keep_edge_max(signal: &RowSignals, edge: &mut [u8]) {
    for (index, value) in signal.edge.iter().enumerate() {
        edge[index] = edge[index].max(*value);
    }
}

struct ReaderBootstrap {
    reader: Retained<AVAssetReader>,
    output: Retained<AVAssetReaderTrackOutput>,
    _track: Retained<AVAssetTrack>,
}
