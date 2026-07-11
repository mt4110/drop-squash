mod cache;
mod lemonsqueezy;
mod provider;
mod trial;

pub use cache::{license_key_fingerprint, LicenseCache};
pub use lemonsqueezy::LemonSqueezyProvider;
pub use provider::{LicenseActivation, LicenseProvider};
pub use trial::{trial_state_from_history, LicenseGate};
