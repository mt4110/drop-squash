mod app;
mod commerce;
mod distribution;

pub(super) fn has_required_detail(blocker: &str, action: &str) -> bool {
    required_phrases(blocker)
        .iter()
        .all(|phrase| action.contains(phrase))
}

fn required_phrases(blocker: &str) -> &'static [&'static str] {
    app::for_blocker(blocker)
        .or_else(|| commerce::for_blocker(blocker))
        .or_else(|| distribution::for_blocker(blocker))
        .unwrap_or(&[])
}
