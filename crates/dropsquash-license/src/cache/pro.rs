use crate::LicenseCache;

impl LicenseCache {
    pub fn permits_pro(&self, now_unix: u64) -> bool {
        self.valid
            && self.has_activation_identity()
            && self
                .offline_grace_until_unix
                .is_some_and(|until| now_unix <= until)
    }

    fn has_activation_identity(&self) -> bool {
        has_value(&self.instance_id)
            && self
                .license_key_fingerprint
                .as_deref()
                .is_some_and(is_fingerprint)
    }
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|text| !text.trim().is_empty())
}

fn is_fingerprint(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|char| char.is_ascii_hexdigit())
}
