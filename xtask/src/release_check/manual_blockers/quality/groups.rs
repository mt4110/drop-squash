mod app;
mod distribution;
mod license;

type Groups = &'static [&'static [&'static str]];

pub(super) fn for_check(check: &str) -> Option<Groups> {
    app::for_check(check)
        .or_else(|| license::for_check(check))
        .or_else(|| distribution::for_check(check))
}
