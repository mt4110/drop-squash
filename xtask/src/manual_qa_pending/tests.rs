use super::{parse_args, pending_rows, USAGE};

#[test]
fn finds_pending_result_rows() {
    let rows = pending_rows(
        "| App build | DropSquash 0.1.0 git abc1234 |\n\
| Choose recording conversion | Small `.mov` screen recording | Creates smaller `.squashed.mp4`; original remains |  |\n\
| Benchmark sample set | Three samples | recorded |\n",
    );

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "Choose recording conversion");
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}
