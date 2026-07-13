use super::done_rows_with_incomplete_markers;

#[test]
fn accepts_done_rows_without_incomplete_markers() {
    let text = "| 1 | Persist settings | Done | Stored safely | Verified by tests |\n";

    assert!(done_rows_with_incomplete_markers(text).is_empty());
}

#[test]
fn reports_done_rows_with_remaining_work() {
    let text = "| 6 Release pipeline | Done | Ship beta | publication remains |\n";

    assert_eq!(
        done_rows_with_incomplete_markers(text),
        vec!["6 Release pipeline"]
    );
}

#[test]
fn reports_backlog_done_rows_with_remaining_work() {
    let text = "| 11 | Build release pipeline | Done | signed publication remains |\n";

    assert_eq!(
        done_rows_with_incomplete_markers(text),
        vec!["Build release pipeline"]
    );
}

#[test]
fn ignores_in_progress_rows_with_remaining_work() {
    let text = "| 6 Release pipeline | In progress | Ship beta | publication remains |\n";

    assert!(done_rows_with_incomplete_markers(text).is_empty());
}

#[test]
fn accepts_done_rows_describing_current_limits() {
    let text = "| 5 | Split files | Done | TS/TSX remain <= 512 lines |\n";

    assert!(done_rows_with_incomplete_markers(text).is_empty());
}

#[test]
fn productization_doc_does_not_overclaim_done_rows() {
    let text = std::fs::read_to_string("../docs/productization.md").unwrap();

    assert!(done_rows_with_incomplete_markers(&text).is_empty());
}
