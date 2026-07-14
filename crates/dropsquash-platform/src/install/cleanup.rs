use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallerCleanup {
    pub mounted_volume_path: Option<PathBuf>,
    pub downloaded_dmg_path: Option<PathBuf>,
    pub should_offer_mounted_volume_eject: bool,
    pub should_offer_downloaded_dmg_trash: bool,
}

pub fn cleanup_after_applications_install(source: &Path, target: &Path) -> InstallerCleanup {
    let mounted_volume_path = mounted_volume_path(source);
    let installed_in_applications = path_has_prefix(target, "/Applications/");
    let should_offer_mounted_volume_eject =
        mounted_volume_path.is_some() && installed_in_applications;
    InstallerCleanup {
        mounted_volume_path,
        downloaded_dmg_path: None,
        should_offer_mounted_volume_eject,
        should_offer_downloaded_dmg_trash: false,
    }
}

fn mounted_volume_path(path: &Path) -> Option<PathBuf> {
    let path = path.to_string_lossy().replace('\\', "/");
    let rest = path.strip_prefix("/Volumes/")?;
    let volume_name = rest.split('/').next()?;
    if volume_name.is_empty() {
        return None;
    }
    Some(Path::new("/Volumes").join(volume_name))
}

fn path_has_prefix(path: &Path, prefix: &str) -> bool {
    path.to_string_lossy()
        .replace('\\', "/")
        .starts_with(prefix)
}
