use dropsquash_core::{OutputSize, Profile, SourcePolicy};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileOption {
    pub value: Profile,
    pub label: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSizeOption {
    pub value: OutputSize,
    pub label: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcePolicyOption {
    pub value: SourcePolicy,
    pub label: &'static str,
}

pub fn profile_options(values: [Profile; 8]) -> Vec<ProfileOption> {
    values
        .into_iter()
        .map(|value| ProfileOption {
            value,
            label: value.display_name(),
        })
        .collect()
}

pub fn size_options(values: [OutputSize; 4]) -> Vec<OutputSizeOption> {
    values
        .into_iter()
        .map(|value| OutputSizeOption {
            value,
            label: value.display_name(),
        })
        .collect()
}

pub fn source_policy_options() -> Vec<SourcePolicyOption> {
    [
        (SourcePolicy::Ask, "Ask after saving"),
        (SourcePolicy::Keep, "Keep original"),
        (SourcePolicy::Trash, "Move to Trash"),
    ]
    .into_iter()
    .map(|(value, label)| SourcePolicyOption { value, label })
    .collect()
}
