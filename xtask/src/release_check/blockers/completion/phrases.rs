mod app;
mod commerce;
mod distribution;

type Entry = (&'static str, &'static str);

const GROUPS: &[&[Entry]] = &[app::ENTRIES, commerce::ENTRIES, distribution::ENTRIES];

pub(super) fn for_blocker(blocker: &str) -> impl Iterator<Item = &'static str> + '_ {
    GROUPS
        .iter()
        .flat_map(|group| group.iter())
        .filter(move |(candidate, _)| *candidate == blocker)
        .map(|(_, phrase)| *phrase)
}
