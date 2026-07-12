mod core_docs;
mod distribution_docs;
mod qa_evidence;

type Entry = (&'static str, &'static str);

const GROUPS: &[&[Entry]] = &[
    core_docs::ENTRIES,
    qa_evidence::ENTRIES,
    distribution_docs::ENTRIES,
];

pub(super) fn required_text() -> impl Iterator<Item = Entry> {
    GROUPS.iter().flat_map(|group| group.iter().copied())
}
