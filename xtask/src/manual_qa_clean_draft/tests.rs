use super::{clean, parse_args, USAGE};

#[test]
fn keeps_only_table_rows() {
    let cleaned = clean(
        "Prepared manual QA draft only. Replace this file with concrete observations.\n\n```sh\ncmd\n```\n| App build | DropSquash 0.1.0 git abc1234 |\n| Check | Expected | Result |\n",
    )
    .unwrap();

    assert_eq!(
        cleaned,
        "| App build | DropSquash 0.1.0 git abc1234 |\n| Check | Expected | Result |\n"
    );
}

#[test]
fn rejects_non_prepared_input() {
    assert!(clean("| App build | x |\n")
        .unwrap_err()
        .contains("prepared draft"));
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}
