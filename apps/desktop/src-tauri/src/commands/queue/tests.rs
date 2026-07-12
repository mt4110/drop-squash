use dropsquash_core::{OutputSize, Profile, SourcePolicy};

use super::job_from_request;
use crate::commands::dto::ConvertRequest;

#[test]
fn queue_job_uses_conversion_request_settings() {
    let job = job_from_request(ConvertRequest {
        input_path: "/tmp/in.mov".into(),
        output_dir: "/tmp/out".into(),
        profile: Profile::Docs,
        output_size: OutputSize::P720,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: true,
    });

    assert_eq!(job.input_path, std::path::Path::new("/tmp/in.mov"));
    assert_eq!(job.output_dir, std::path::Path::new("/tmp/out"));
    assert_eq!(job.profile, Profile::Docs);
    assert_eq!(job.output_size, OutputSize::P720);
    assert_eq!(job.source_policy, SourcePolicy::Ask);
}
