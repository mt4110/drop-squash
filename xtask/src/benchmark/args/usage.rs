pub(super) fn text() -> String {
    concat!(
        "usage: cargo run -p xtask -- benchmark ",
        "--input <movie> --output-dir <dir> ",
        "[--profile auto] [--size auto] [--release-set]"
    )
    .to_string()
}
