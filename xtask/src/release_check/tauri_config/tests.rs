use super::check;

#[test]
fn accepts_required_distribution_metadata() {
    let (_directory, path) = write_config(
        r#"{
  "productName": "DropSquash",
  "identifier": "io.github.mt4110.dropsquash",
  "app": {
    "security": {
      "csp": {
        "connect-src": "ipc: http://ipc.localhost"
      }
    }
  },
  "bundle": {
    "targets": ["app", "dmg"]
  }
}"#,
    );

    assert!(check(&path).is_ok());
}

#[test]
fn rejects_changed_bundle_identifier() {
    let (_directory, path) = write_config(
        r#"{
  "productName": "DropSquash",
  "identifier": "com.example.changed",
  "app": {
    "security": {
      "csp": {
        "connect-src": "ipc: http://ipc.localhost"
      }
    }
  },
  "bundle": {
    "targets": ["app", "dmg"]
  }
}"#,
    );

    let error = check(&path).unwrap_err();

    assert!(error.contains("identifier"));
}

#[test]
fn rejects_missing_dmg_target() {
    let (_directory, path) = write_config(
        r#"{
  "productName": "DropSquash",
  "identifier": "io.github.mt4110.dropsquash",
  "app": {
    "security": {
      "csp": {
        "connect-src": "ipc: http://ipc.localhost"
      }
    }
  },
  "bundle": {
    "targets": ["app"]
  }
}"#,
    );

    let error = check(&path).unwrap_err();

    assert!(error.contains("dmg"));
}

fn write_config(text: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tauri.conf.json");
    std::fs::write(&path, text).unwrap();
    (directory, path)
}
