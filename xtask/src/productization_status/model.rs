#[derive(Debug, Eq, PartialEq)]
pub(super) struct Blocker {
    pub(super) name: String,
    pub(super) status: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct EvidenceAction {
    pub(super) blocker: String,
    pub(super) next_action: String,
    pub(super) owner: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Track {
    pub(super) order: usize,
    pub(super) name: String,
    pub(super) blockers: Vec<String>,
    pub(super) record_target: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct TrackStatus {
    pub(super) order: usize,
    pub(super) name: String,
    pub(super) blockers: Vec<String>,
    pub(super) remaining: Vec<String>,
    pub(super) record_target: String,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Report {
    pub(super) total: usize,
    pub(super) verified: usize,
    pub(super) blocked: usize,
    pub(super) actions: Vec<EvidenceAction>,
    pub(super) tracks: Vec<TrackStatus>,
}
