pub(super) fn text() -> String {
    concat!(
        "usage: cargo run -p xtask -- benchmark ",
        "--input <movie> --output-dir <dir> ",
        "[--csv-output <csv>] [--profile auto] [--size auto] [--release-set]"
    )
    .to_string()
}
