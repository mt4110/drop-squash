use super::{cache, parse_args, INSTANCE_ID, INSTANCE_NAME, USAGE};

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn accepts_expired_flag_in_any_position() {
    assert_eq!(
        parse_args(vec!["/tmp/license.json".into(), "--expired".into()]).unwrap(),
        ("/tmp/license.json".into(), true)
    );
    assert_eq!(
        parse_args(vec!["--expired".into(), "/tmp/license.json".into()]).unwrap(),
        ("/tmp/license.json".into(), true)
    );
}

#[test]
fn builds_expected_expired_cache() {
    let cache = cache(true);
    assert_eq!(cache.instance_name.as_deref(), Some(INSTANCE_NAME));
    assert_eq!(cache.instance_id.as_deref(), Some(INSTANCE_ID));
    assert_eq!(cache.offline_grace_until_unix, Some(101));
    assert!(cache.valid);
}
