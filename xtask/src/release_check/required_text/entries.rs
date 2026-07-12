mod core_docs;
mod distribution_docs;
mod qa_evidence;
mod qa_evidence_distribution;

type Entry = (&'static str, &'static str);

const GROUPS: &[&[Entry]] = &[
    core_docs::ENTRIES,
    qa_evidence::ENTRIES,
    qa_evidence_distribution::ENTRIES,
    distribution_docs::ENTRIES,
];

pub(super) fn required_text() -> impl Iterator<Item = Entry> {
    GROUPS.iter().flat_map(|group| group.iter().copied())
}
