use std::path::{Path, PathBuf};

use crate::artifact_check;

pub fn run(args: Vec<String>) -> Result<(), String> {
    let request = Request::parse(args)?;
    check(&request)?;
    println!("signed DMG artifact checks passed");
    Ok(())
}

fn check(request: &Request) -> Result<(), String> {
    reject_same_path(&request.signed, &request.unsigned)?;
    let signed = artifact_check::read_checked(&request.signed, "signed DMG")?;
    let unsigned = artifact_check::read_checked(&request.unsigned, "unsigned DMG")?;
    if signed == unsigned {
        return Err("signed DMG must differ from the unsigned input".to_string());
    }
    Ok(())
}

fn reject_same_path(signed: &Path, unsigned: &Path) -> Result<(), String> {
    let signed = signed.canonicalize().map_err(|error| error.to_string())?;
    let unsigned = unsigned.canonicalize().map_err(|error| error.to_string())?;
    if signed == unsigned {
        return Err("signed DMG must not be the unsigned input path".to_string());
    }
    Ok(())
}

struct Request {
    signed: PathBuf,
    unsigned: PathBuf,
}

impl Request {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        match args.as_slice() {
            [signed, unsigned] => Ok(Self {
                signed: PathBuf::from(signed),
                unsigned: PathBuf::from(unsigned),
            }),
            _ => Err(
                "signed-dmg-check requires <signed DropSquash.dmg> <unsigned DropSquash.dmg>"
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
