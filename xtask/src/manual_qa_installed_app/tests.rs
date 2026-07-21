use super::{exists_label, mounted_dmg_label, restore, stash};

#[test]
fn stashes_and_restores_app_bundle() {
    let temp = tempfile::tempdir().unwrap();
    let apps = temp.path().join("Applications");
    let backup = temp.path().join("backup");
    let app = apps.join("DropSquash.app");
    std::fs::create_dir_all(&app).unwrap();

    let moved = stash(&app, &backup).unwrap();
    assert_eq!(moved, backup.join("DropSquash.app"));
    assert!(!app.exists());

    let restored = restore(&app, &backup).unwrap();
    assert_eq!(restored, app);
    assert!(app.exists());
}

#[test]
fn refuses_to_overwrite_existing_backup_or_install() {
    let temp = tempfile::tempdir().unwrap();
    let apps = temp.path().join("Applications");
    let backup = temp.path().join("backup");
    let app = apps.join("DropSquash.app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(backup.join("DropSquash.app")).unwrap();

    let stash_error = stash(&app, &backup).unwrap_err();
    assert!(stash_error.contains("backup app already exists"));

    std::fs::remove_dir_all(&app).unwrap();
    std::fs::create_dir_all(&app).unwrap();
    let restore_error = restore(&app, &backup).unwrap_err();
    assert!(restore_error.contains("installed app already exists"));
}

#[test]
fn reports_presence_labels() {
    let temp = tempfile::tempdir().unwrap();
    let present = temp.path().join("present.app");
    std::fs::create_dir_all(&present).unwrap();
    let missing = temp.path().join("missing.app");

    assert_eq!(exists_label(&present), "present");
    assert_eq!(exists_label(&missing), "missing");
}

#[test]
fn reports_mounted_dmg_readiness() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("DropSquash.app");
    let backup = temp.path().join("backup/DropSquash.app");

    assert_eq!(mounted_dmg_label(&app, &backup), "ready");
    std::fs::create_dir_all(&app).unwrap();
    assert_eq!(mounted_dmg_label(&app, &backup), "stash-required");
    std::fs::create_dir_all(backup.parent().unwrap()).unwrap();
    std::fs::create_dir_all(&backup).unwrap();
    assert_eq!(mounted_dmg_label(&app, &backup), "restore-before-stash");
}
