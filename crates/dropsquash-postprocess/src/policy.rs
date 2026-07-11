use dropsquash_core::SourcePolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePolicyConfig {
    pub policy: SourcePolicy,
}

impl Default for SourcePolicyConfig {
    fn default() -> Self {
        Self {
            policy: SourcePolicy::Ask,
        }
    }
}
