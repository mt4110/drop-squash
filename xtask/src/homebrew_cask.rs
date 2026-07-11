mod validation;

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
        validation::require_version(&self.version)?;
        validation::require_dmg_url(&self.url)?;
        validation::require_versioned_url(&self.version, &self.url)?;
        validation::require_sha256(&self.sha256)?;
        validation::require_https_url(&self.homepage)?;
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

#[cfg(test)]
mod tests;
