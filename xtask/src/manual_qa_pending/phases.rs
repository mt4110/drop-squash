pub(super) fn for_label(label: &str) -> Option<&'static str> {
    match label {
        "Disk image launch notice" => Some("Mounted DMG"),
        "Choose recording conversion" | "Drag-and-drop conversion"
        | "Privacy receipt sidecar" | "Reveal privacy receipt" | "Ask source policy"
        | "Trash source policy" | "Reveal output" => Some("Small Sample"),
        "Duplicate output naming" => Some("Duplicate Sample"),
        "Cancellation" | "Larger output" => Some("Large Sample"),
        "Multi-file queue" | "Queued job cancellation" | "Batch summary" => Some("Queue Sample"),
        "Failed conversion" => Some("Custom Failure Input"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::for_label;

    #[test]
    fn maps_queue_rows_to_queue_phase() {
        assert_eq!(for_label("Batch summary"), Some("Queue Sample"));
    }
}
