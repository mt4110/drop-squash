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

pub fn profile_options(values: &[Profile]) -> Vec<ProfileOption> {
    values
        .iter()
        .copied()
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
        (SourcePolicy::Ask, "保存後に確認 / Ask after save"),
        (SourcePolicy::Keep, "元を残す / Keep original"),
        (SourcePolicy::Trash, "ゴミ箱へ移動 / Move to Trash"),
    ]
    .into_iter()
    .map(|(value, label)| SourcePolicyOption { value, label })
    .collect()
}
