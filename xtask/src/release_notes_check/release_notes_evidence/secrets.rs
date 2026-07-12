pub(super) fn validate(text: &str) -> Vec<String> {
    crate::secret_text::violations("release notes", text)
}
