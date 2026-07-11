pub fn run(args: Vec<String>) -> Result<(), String> {
    let input = Input::parse(args)?;
    println!("{}", render(&input));
    Ok(())
}

struct Input {
    version: String,
    url: String,
    sha256: String,
    homepage: String,
}

impl Input {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        if args.len() != 4 {
            return Err("homebrew-cask requires <version> <url> <sha256> <homepage>".to_string());
        }
        let input = Self {
            version: args[0].clone(),
            url: args[1].clone(),
            sha256: args[2].clone(),
            homepage: args[3].clone(),
        };
        input.validate()?;
        Ok(input)
    }

    fn validate(&self) -> Result<(), String> {
        require_version(&self.version)?;
        require_dmg_url(&self.url)?;
        require_versioned_url(&self.version, &self.url)?;
        require_sha256(&self.sha256)?;
        require_https_url(&self.homepage)?;
        Ok(())
    }
}

fn render(input: &Input) -> String {
    format!(
        r#"cask "dropsquash" do
  version "{version}"
  sha256 "{sha256}"

  url "{url}"
  name "DropSquash"
  desc "Local screen recording compressor"
  homepage "{homepage}"

  app "DropSquash.app"
end"#,
        version = input.version,
        url = input.url,
        sha256 = input.sha256,
        homepage = input.homepage
    )
}

fn require_clean(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.chars().any(char::is_whitespace) {
        return Err(format!(
            "{label} must be non-empty and contain no whitespace"
        ));
    }
    Ok(())
}

fn require_version(version: &str) -> Result<(), String> {
    require_clean("version", version)?;
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 && parts.iter().all(|part| is_numeric_part(part)) {
        return Ok(());
    }
    Err("version must use major.minor.patch digits".to_string())
}

fn is_numeric_part(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|char| char.is_ascii_digit())
}

fn require_dmg_url(url: &str) -> Result<(), String> {
    require_https_url(url)?;
    if url.ends_with(".dmg") {
        return Ok(());
    }
    Err("url must point to a .dmg file".to_string())
}

fn require_versioned_url(version: &str, url: &str) -> Result<(), String> {
    if url.contains(&format!("/v{version}/")) {
        return Ok(());
    }
    Err("url must point to the matching v<version> release".to_string())
}

fn require_https_url(url: &str) -> Result<(), String> {
    require_clean("url", url)?;
    if url.contains("example.com") {
        return Err("url must not contain example.com".to_string());
    }
    if url.starts_with("https://") {
        return Ok(());
    }
    Err("url must start with https://".to_string())
}

fn require_sha256(value: &str) -> Result<(), String> {
    require_clean("sha256", value)?;
    if value.len() == 64 && value.chars().all(|char| char.is_ascii_hexdigit()) {
        return Ok(());
    }
    Err("sha256 must be 64 hex characters".to_string())
}

#[cfg(test)]
mod tests;
