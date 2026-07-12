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
        has_value(&self.instance_id) && has_value(&self.license_key_fingerprint)
    }
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|text| !text.trim().is_empty())
}
