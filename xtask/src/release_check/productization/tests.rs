use super::done_rows_with_incomplete_markers;

#[test]
fn accepts_done_rows_without_incomplete_markers() {
    let text = "| 1 | Persist settings | Done | Stored safely | Verified by tests |\n";

    assert!(done_rows_with_incomplete_markers(text).is_empty());
}

#[test]
fn reports_done_rows_with_remaining_work() {
    let text = "| 6 Release pipeline | Done | Ship beta | signing remains |\n";

    assert_eq!(
        done_rows_with_incomplete_markers(text),
        vec!["6 Release pipeline"]
    );
}

#[test]
fn ignores_in_progress_rows_with_remaining_work() {
    let text = "| 6 Release pipeline | In progress | Ship beta | signing remains |\n";

    assert!(done_rows_with_incomplete_markers(text).is_empty());
}

#[test]
fn productization_doc_does_not_overclaim_done_rows() {
    let text = std::fs::read_to_string("../docs/productization.md").unwrap();

    assert!(done_rows_with_incomplete_markers(&text).is_empty());
}
