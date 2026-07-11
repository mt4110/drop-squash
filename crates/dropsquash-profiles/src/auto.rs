use dropsquash_core::{MediaInfo, Profile};

pub fn resolve_profile(requested: Profile, media: &MediaInfo) -> Profile {
    if requested != Profile::Auto {
        return requested;
    }

    if media.width.is_some_and(|width| width > 1920) {
        return Profile::Slack;
    }

    Profile::Docs
}
