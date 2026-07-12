use super::{classification_rows, release_blocker_rows};

#[test]
fn accepts_single_release_blocker_rows() {
    let text = "| Signed DMG | Blocked | Evidence required | TBD | Release notes |\n";

    assert!(release_blocker_rows(text).is_empty());
}

#[test]
fn reports_duplicate_release_blocker_rows() {
    let text = [
        "| Signed DMG | Blocked | Evidence required | TBD | Release notes |\n",
        "| Signed DMG | Verified | Evidence captured | Release notes | Release notes |\n",
    ]
    .join("");

    let duplicates = release_blocker_rows(&text);

    assert_eq!(duplicates, vec!["Signed DMG"]);
}

#[test]
fn accepts_single_classification_rows() {
    let text = "| Signed DMG | Signing/notarization | Sign the public DMG | Release notes |\n";

    assert!(classification_rows(text).is_empty());
}

#[test]
fn reports_duplicate_classification_rows() {
    let text = [
        "| Signed DMG | Signing/notarization | Sign the public DMG | Release notes |\n",
        "| Signed DMG | Signing/notarization | Capture codesign evidence | Release notes |\n",
    ]
    .join("");

    let duplicates = classification_rows(&text);

    assert_eq!(duplicates, vec!["Signed DMG"]);
}
