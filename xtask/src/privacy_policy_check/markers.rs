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
    "curl::",
    "fetch(",
    "hyper::",
    "isahc::",
    "native_tls",
    "reqwest",
    "surf::",
    "tcpstream",
    "tokio_tungstenite",
    "tungstenite",
    "ureq::",
    "websocket",
];
