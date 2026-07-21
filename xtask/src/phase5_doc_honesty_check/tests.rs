use super::check;

#[test]
fn rejects_strong_pending_row() {
    let doc = "| Exposure path | Classification | Strongest current evidence |\n\
               | --- | --- | --- |\n\
               | typed | covered | rerun pending |";
    assert!(check(doc).is_err());
}

#[test]
fn accepts_detected_pending_row() {
    let doc = "| Exposure path | Classification | Strongest current evidence |\n\
               | --- | --- | --- |\n\
               | typed | detected_but_not_covered | rerun pending |";
    assert!(check(doc).is_ok());
}

#[test]
fn rejects_non_negated_leak_zero_claim() {
    assert!(check("DropSquash is leak-zero for QA teams.").is_err());
}

#[test]
fn accepts_negated_leak_zero_boundary() {
    assert!(check("This is not a leak-zero claim.").is_ok());
}

#[test]
fn accepts_unsafe_claims_list() {
    let doc = "Unsafe claims:\n\n- leak-zero\n- enterprise audit-ready";
    assert!(check(doc).is_ok());
}
