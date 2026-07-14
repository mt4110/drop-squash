mod agent_docs;
mod core_docs;
mod distribution_docs;
mod native_docs;
mod productization_docs;
mod qa_evidence;
mod qa_evidence_artifacts;
mod qa_evidence_distribution;
mod qa_evidence_manual;
mod qa_evidence_publish;
mod qa_evidence_release_blockers;
mod qa_evidence_release_notes;
mod qa_evidence_release_template;
mod release_docs;
mod release_signing_docs;
mod signed_publication_docs;
mod website_docs;

type Entry = (&'static str, &'static str);

const GROUPS: &[&[Entry]] = &[
    agent_docs::ENTRIES,
    core_docs::ENTRIES,
    native_docs::ENTRIES,
    productization_docs::CORE,
    productization_docs::RELEASE,
    qa_evidence::CORE,
    qa_evidence::SITE,
    qa_evidence_artifacts::ENTRIES,
    qa_evidence_release_blockers::ENTRIES,
    qa_evidence_manual::ENTRIES,
    qa_evidence_release_notes::ENTRIES,
    qa_evidence_release_template::ENTRIES,
    qa_evidence_publish::ENTRIES,
    qa_evidence_distribution::ENTRIES,
    distribution_docs::CORE,
    distribution_docs::LICENSE,
    release_docs::ENTRIES,
    release_signing_docs::ENTRIES,
    signed_publication_docs::ENTRIES,
    website_docs::ENTRIES,
];

pub(super) fn required_text() -> impl Iterator<Item = Entry> {
    GROUPS.iter().flat_map(|group| group.iter().copied())
}
