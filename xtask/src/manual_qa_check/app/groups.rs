mod app;
mod license;
mod privacy;
mod queue;
mod release;
mod source;

pub(super) fn for_label(label: &str) -> Option<&'static [&'static [&'static str]]> {
    app::for_label(label)
        .or_else(|| privacy::for_label(label))
        .or_else(|| queue::for_label(label))
        .or_else(|| source::for_label(label))
        .or_else(|| license::for_label(label))
        .or_else(|| release::for_label(label))
}
