use super::{run, success_lines};

#[test]
fn rejects_extra_arguments() {
    let error = run(vec!["extra".into()]).unwrap_err();
    assert!(error.contains("public-web-ready"));
}

#[test]
fn prints_local_public_web_success_lines() {
    let lines = success_lines();
    assert!(lines.iter().any(|line| line.contains("website-check")));
    assert!(lines.iter().any(|line| line.contains("verify:site")));
    assert!(lines
        .iter()
        .any(|line| line.contains(&crate::current_date::display())));
}
