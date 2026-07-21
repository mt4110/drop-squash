use std::process::Command;

const FALLBACK: &str = "current date unavailable";

pub(crate) fn display() -> String {
    let output = Command::new("date").arg("+%A, %B %-d, %Y").output();
    let Ok(output) = output else {
        return FALLBACK.to_string();
    };
    if !output.status.success() {
        return FALLBACK.to_string();
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        FALLBACK.to_string()
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::{display, FALLBACK};

    #[test]
    fn returns_non_empty_date_text() {
        let text = display();
        assert!(!text.is_empty());
        assert_ne!(text, FALLBACK);
        assert!(text.contains(','));
    }
}
