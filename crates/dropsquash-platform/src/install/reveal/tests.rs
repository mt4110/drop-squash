use std::path::Path;

use super::reveal_finder_item;

#[test]
fn rejects_missing_finder_item_before_platform_call() {
    let error = reveal_finder_item(Path::new("/tmp/dropsquash-missing-output.mp4")).unwrap_err();

    assert!(error.to_string().contains("finder item does not exist"));
}
