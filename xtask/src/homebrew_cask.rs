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
        require_clean("version", &self.version)?;
        require_https_url(&self.url)?;
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

fn require_https_url(url: &str) -> Result<(), String> {
    require_clean("url", url)?;
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
