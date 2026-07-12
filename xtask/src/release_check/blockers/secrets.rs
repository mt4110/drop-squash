pub(super) fn values(text: &str) -> Vec<String> {
    crate::secret_text::violations("release blockers", text)
}
