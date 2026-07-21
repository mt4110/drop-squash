use std::str::FromStr;

use clap::ValueEnum;
use dropsquash_core::{MaskMode, MaskRect};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum MaskModeArg {
    #[value(name = "solid_black")]
    SolidBlack,
    #[value(name = "black_noise")]
    BlackNoise,
}

impl From<MaskModeArg> for MaskMode {
    fn from(value: MaskModeArg) -> Self {
        match value {
            MaskModeArg::SolidBlack => Self::SolidBlack,
            MaskModeArg::BlackNoise => Self::BlackNoise,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaskRectArg(pub MaskRect);

impl From<MaskRectArg> for MaskRect {
    fn from(value: MaskRectArg) -> Self {
        value.0
    }
}

impl FromStr for MaskRectArg {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.split(',').collect::<Vec<_>>();
        let [x, y, width, height] = parts.as_slice() else {
            return Err("secure-share rect must be x,y,width,height".to_string());
        };
        Ok(Self(MaskRect {
            x: parse_part(x, "x")?,
            y: parse_part(y, "y")?,
            width: parse_part(width, "width")?,
            height: parse_part(height, "height")?,
        }))
    }
}

fn parse_part(value: &str, name: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|_| format!("secure-share rect {name} must be an unsigned integer"))
}

#[cfg(test)]
mod tests {
    use super::MaskRectArg;

    #[test]
    fn parses_mask_rect() {
        let rect = "10,20,300,400".parse::<MaskRectArg>().unwrap().0;
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 300);
        assert_eq!(rect.height, 400);
    }

    #[test]
    fn rejects_wrong_part_count() {
        let err = "10,20,30".parse::<MaskRectArg>().unwrap_err();
        assert_eq!(err, "secure-share rect must be x,y,width,height");
    }
}
