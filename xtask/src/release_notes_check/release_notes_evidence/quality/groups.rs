mod app;
mod distribution;
mod license;

type Groups = &'static [&'static [&'static str]];

pub(super) fn for_label(label: &str) -> Option<Groups> {
    app::for_label(label)
        .or_else(|| license::for_label(label))
        .or_else(|| distribution::for_label(label))
}
