pub(super) const PRODUCT_SETUP: &[&str] = &[
    "sandbox",
    "dropsquash",
    "intended product",
    "license keys enabled",
];
pub(super) const PURCHASE: &[&str] = &["sandbox", "intended product", "test buyer", "order"];

pub(super) const EMPTY_KEY_CACHE: &[&str] = &["raw key", "no fingerprint", "no instance"];
pub(super) const EMPTY_KEY_ACTION: &[&str] = &["activate", "disabled"];

pub(super) const INVALID_KEY_CACHE: &[&str] =
    &["friendly", "raw key", "no fingerprint", "no instance"];
pub(super) const VALID_KEY_CACHE: &[&str] = &[
    "lemon squeezy",
    "sandbox",
    "pro",
    "fingerprint",
    "instance",
    "raw key",
];
pub(super) const ACTIVATING_ACTION: &[&str] = &["activating", "disabled"];

pub(super) const NETWORK_FAILURE_CACHE: &[&str] = &[
    "friendly",
    "network",
    "existing",
    "valid",
    "preserved",
    "fingerprint",
    "instance",
    "raw key",
];

pub(super) const EXPIRED_REFRESH: &[&str] = &[
    "expired",
    "offline grace",
    "reconnect",
    "blocked",
    "before starting",
    "raw key",
];

pub(super) const FORGET_ACTION: &[&str] = &["forgetting", "disabled"];
