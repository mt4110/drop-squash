use std::path::PathBuf;

use dropsquash_privacy::PrivacyReceipt;

pub fn run(output: PathBuf) -> dropsquash_core::Result<()> {
    for line in receipt_lines(output)? {
        println!("{line}");
    }
    Ok(())
}

fn receipt_lines(output: PathBuf) -> dropsquash_core::Result<Vec<String>> {
    let (path, receipt) = PrivacyReceipt::load_for_output(&output)?;
    Ok(format_receipt(path, receipt))
}

fn format_receipt(path: PathBuf, receipt: PrivacyReceipt) -> Vec<String> {
    vec![
        format!("privacy receipt: {}", path.display()),
        format!("input file name: {}", receipt.input_name),
        format!("output file name: {}", receipt.output_name),
        format!("uploaded bytes: {}", receipt.uploaded_bytes),
        format!("metadata policy: {}", receipt.metadata_policy.as_str()),
    ]
}

#[cfg(test)]
mod tests {
    use dropsquash_privacy::MetadataPolicy;

    use super::*;

    #[test]
    fn formats_receipt_without_absolute_media_paths() {
        let lines = format_receipt(
            PathBuf::from("/tmp/out/movie.privacy.json"),
            PrivacyReceipt {
                input_name: "secret.mov".to_string(),
                output_name: "movie.mp4".to_string(),
                uploaded_bytes: 0,
                metadata_policy: MetadataPolicy::Preserve,
            },
        );

        assert!(lines.contains(&"input file name: secret.mov".to_string()));
        assert!(lines.contains(&"uploaded bytes: 0".to_string()));
        assert!(lines.contains(&"metadata policy: preserve".to_string()));
        assert!(!lines.iter().any(|line| line.contains("/Users/me/Secret")));
    }
}
