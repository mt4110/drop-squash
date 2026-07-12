use std::path::Path;

pub(super) struct Issues<'a> {
    pub missing: Vec<&'a str>,
    pub invalid: Vec<&'a str>,
    pub unproven: Vec<&'a str>,
    pub stale: Vec<&'a str>,
    pub misplaced_ref: Vec<&'a str>,
    pub incomplete: Vec<&'a str>,
    pub misplaced: Vec<&'a str>,
    pub mismatched_urls: Vec<&'a str>,
    pub unclassified: Vec<&'a str>,
    pub unknown_classifications: Vec<&'a str>,
    pub duplicate_rows: Vec<&'a str>,
    pub duplicate_classifications: Vec<&'a str>,
    pub secret_values: Vec<String>,
}

impl Issues<'_> {
    pub(super) fn is_empty(&self) -> bool {
        self.missing.is_empty()
            && self.invalid.is_empty()
            && self.unproven.is_empty()
            && self.stale.is_empty()
            && self.misplaced_ref.is_empty()
            && self.incomplete.is_empty()
            && self.misplaced.is_empty()
            && self.mismatched_urls.is_empty()
            && self.unclassified.is_empty()
            && self.unknown_classifications.is_empty()
            && self.duplicate_rows.is_empty()
            && self.duplicate_classifications.is_empty()
            && self.secret_values.is_empty()
    }

    pub(super) fn format(self, path: &Path) -> String {
        format!(
            "{} has release blocker issues: {}{}{}{}{}{}{}{}{}{}{}{}{}",
            path.display(),
            join_prefix("missing ", self.missing),
            join_prefix(" invalid status ", self.invalid),
            join_prefix(" unproven verified ", self.unproven),
            join_prefix(" stale blocked ", self.stale),
            join_prefix(" misplaced verified reference ", self.misplaced_ref),
            join_prefix(" incomplete requirement ", self.incomplete),
            join_prefix(" misplaced record target ", self.misplaced),
            join_prefix(" mismatched URL pair ", self.mismatched_urls),
            join_prefix(" unclassified ", self.unclassified),
            join_prefix(" unknown classification ", self.unknown_classifications),
            join_prefix(" duplicate release blocker row ", self.duplicate_rows),
            join_prefix(
                " duplicate evidence classification row ",
                self.duplicate_classifications,
            ),
            join_string_prefix(" secret-like value ", self.secret_values)
        )
    }
}

fn join_prefix(prefix: &str, values: Vec<&str>) -> String {
    if values.is_empty() {
        return String::new();
    }
    format!("{prefix}{}", values.join(", "))
}

fn join_string_prefix(prefix: &str, values: Vec<String>) -> String {
    if values.is_empty() {
        return String::new();
    }
    format!("{prefix}{}", values.join(", "))
}
