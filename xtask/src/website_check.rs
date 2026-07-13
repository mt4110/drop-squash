use std::path::{Path, PathBuf};

mod artifact_files;
mod css_resources;
mod external_policy;
mod href_policy;
mod html_files;
mod html_links;
mod local_links;
mod platform_claims;
mod pre_release_copy;
mod release_copy;
mod required_pages;
mod resource_policy;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let root = match args.as_slice() {
        [] => PathBuf::from("website"),
        [root] => PathBuf::from(root),
        _ => return Err("website-check accepts at most one <website-root>".to_string()),
    };
    let errors = check_root(&root)?;
    if errors.is_empty() {
        println!("website checks passed");
        return Ok(());
    }
    Err(errors.join("\n"))
}

pub(crate) fn check_default_root() -> Result<(), String> {
    check_path(Path::new("website"))
}

pub(crate) fn check_path(root: &Path) -> Result<(), String> {
    let errors = check_root(root)?;
    if errors.is_empty() {
        return Ok(());
    }
    Err(errors.join("\n"))
}

fn check_root(root: &Path) -> Result<Vec<String>, String> {
    let mut errors = Vec::new();
    required_pages::check(root, &mut errors);
    release_copy::check(root, &mut errors);
    artifact_files::check(root, &mut errors)?;
    css_resources::check(root, &mut errors)?;
    for path in html_files::collect(root)? {
        check_html(root, &path, &mut errors)?;
    }
    Ok(errors)
}

fn check_html(root: &Path, path: &Path, errors: &mut Vec<String>) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    for error in crate::secret_text::violations("website", &text) {
        errors.push(format!("{} {error}", path.display()));
    }
    platform_claims::check(path, &text, errors);
    pre_release_copy::check(path, &text, errors);
    if href_policy::has_placeholder_url(&text) {
        errors.push(format!("{} contains placeholder URL", path.display()));
    }
    for href in html_links::hrefs(&text) {
        href_policy::check(path, &href, errors);
        external_policy::check(path, &href, errors);
        if href.starts_with('#') {
            check_fragment(root, path, &href, errors)?;
            continue;
        }
        if href_policy::is_external_or_anchor(&href) {
            continue;
        }
        if !local_links::exists(root, path, &href) {
            errors.push(format!("{} links to missing {href}", path.display()));
            continue;
        }
        check_fragment(root, path, &href, errors)?;
    }
    for src in html_links::srcs(&text) {
        resource_policy::check(root, path, &src, errors);
    }
    for action in html_links::actions(&text) {
        href_policy::check(path, &action, errors);
        external_policy::check(path, &action, errors);
        if !href_policy::is_external_or_anchor(&action) && !local_links::exists(root, path, &action)
        {
            errors.push(format!("{} links to missing {action}", path.display()));
        }
    }
    Ok(())
}

fn check_fragment(
    root: &Path,
    path: &Path,
    href: &str,
    errors: &mut Vec<String>,
) -> Result<(), String> {
    if !local_links::fragment_exists(root, path, href)? {
        errors.push(format!(
            "{} links to missing fragment {href}",
            path.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
