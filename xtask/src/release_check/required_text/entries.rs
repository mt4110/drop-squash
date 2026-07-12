mod agent_docs;
mod core_docs;
mod distribution_docs;
mod productization_docs;
mod qa_evidence;
mod qa_evidence_distribution;
mod qa_evidence_release;
mod release_docs;
mod website_docs;

type Entry = (&'static str, &'static str);

const GROUPS: &[&[Entry]] = &[
    agent_docs::ENTRIES,
    core_docs::ENTRIES,
    productization_docs::ENTRIES,
    qa_evidence::ENTRIES,
    qa_evidence_release::ENTRIES,
    qa_evidence_distribution::ENTRIES,
    distribution_docs::ENTRIES,
    release_docs::ENTRIES,
    website_docs::ENTRIES,
];

pub(super) fn required_text() -> impl Iterator<Item = Entry> {
    GROUPS.iter().flat_map(|group| group.iter().copied())
}
