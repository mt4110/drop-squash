use tokio_util::sync::CancellationToken;

use super::ensure_not_cancelled;

#[test]
fn accepts_uncancelled_token() {
    let cancel = CancellationToken::new();

    assert!(ensure_not_cancelled(&cancel).is_ok());
}

#[test]
fn rejects_cancelled_token_before_postprocessing() {
    let cancel = CancellationToken::new();
    cancel.cancel();

    let error = ensure_not_cancelled(&cancel).unwrap_err();

    assert!(error.contains("cancelled"));
}
