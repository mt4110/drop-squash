use crate::args::{MaskModeArg, MaskRectArg};
use dropsquash_core::{AppError, MaskRect, SecureShareOptions};

pub fn options(
    mode: Option<MaskModeArg>,
    rects: Vec<MaskRectArg>,
) -> dropsquash_core::Result<Option<SecureShareOptions>> {
    match (mode, rects.is_empty()) {
        (None, true) => Ok(None),
        (Some(mode), false) => Ok(Some(SecureShareOptions {
            mask_mode: mode.into(),
            mask_rects: rects.into_iter().map(MaskRect::from).collect(),
            mask_plan: None,
        })),
        (None, false) => Err(AppError::InvalidConfig(
            "secure-share mode is required when secure-share rects are set".to_string(),
        )),
        (Some(_), true) => Err(AppError::InvalidConfig(
            "secure-share needs at least one rect".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::args::{MaskModeArg, MaskRectArg};

    use super::options;

    #[test]
    fn returns_none_without_secure_share_args() {
        assert!(options(None, vec![]).unwrap().is_none());
    }

    #[test]
    fn builds_secure_share_options() {
        let options = options(
            Some(MaskModeArg::SolidBlack),
            vec!["1,2,3,4".parse::<MaskRectArg>().unwrap()],
        )
        .unwrap()
        .unwrap();
        assert_eq!(options.mask_rects.len(), 1);
    }

    #[test]
    fn rejects_rects_without_mode() {
        let err = options(None, vec!["1,2,3,4".parse::<MaskRectArg>().unwrap()])
            .unwrap_err()
            .user_message();
        assert_eq!(
            err,
            "invalid configuration: secure-share mode is required when secure-share rects are set"
        );
    }
}
