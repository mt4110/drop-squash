pub(super) const DISALLOWED: &[&str] = &[
    "posthog",
    "sentry",
    "amplitude",
    "mixpanel",
    "google-analytics",
    "gtag(",
    "sendbeacon",
    "xmlhttprequest",
];

pub(super) const NETWORK: &[&str] = &[
    "@tauri-apps/plugin-http",
    "axios",
    "connect_async",
    "fetch(",
    "reqwest",
    "tcpstream",
    "websocket",
];
