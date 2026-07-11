use super::response::LicenseInstance;
use super::transport::LicenseApiClient;
use super::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[test]
fn activation_uses_instance_id_and_fingerprint_only() {
    let activation = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(true),
            valid: None,
            deactivated: None,
            error: None,
            instance: Some(LicenseInstance {
                id: "instance-1".to_string(),
            }),
        },
    )
    .unwrap();

    assert_eq!(activation.instance_id, "instance-1");
    assert_ne!(activation.license_key_fingerprint, "LS-SECRET-RAW-KEY");
    assert!(activation.valid);
}

#[test]
fn activation_error_is_friendly() {
    let error = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(false),
            valid: None,
            deactivated: None,
            error: Some("Activation limit reached.".to_string()),
            instance: Some(LicenseInstance {
                id: "instance-1".to_string(),
            }),
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("Activation limit reached."));
    assert!(!error.to_string().contains("LS-SECRET-RAW-KEY"));
}

#[test]
fn activation_error_redacts_echoed_license_key() {
    let error = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(false),
            valid: None,
            deactivated: None,
            error: Some("Key LS-SECRET-RAW-KEY is not valid.".to_string()),
            instance: None,
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("[license key]"));
    assert!(!error.to_string().contains("LS-SECRET-RAW-KEY"));
}

#[test]
fn activation_error_redacts_normalized_license_key() {
    let error = activation_from_response(
        "LS-SECRET\nRAW-KEY",
        LicenseApiResponse {
            activated: Some(false),
            valid: None,
            deactivated: None,
            error: Some("Key LS-SECRET RAW-KEY is not valid.".to_string()),
            instance: None,
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("[license key]"));
    assert!(!error.to_string().contains("LS-SECRET RAW-KEY"));
}

#[test]
fn activation_rejects_empty_instance_id() {
    let error = activation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: Some(true),
            valid: None,
            deactivated: None,
            error: None,
            instance: Some(LicenseInstance {
                id: " ".to_string(),
            }),
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("empty instance id"));
}

#[test]
fn deactivation_accepts_confirmed_response() {
    deactivation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: None,
            valid: None,
            deactivated: Some(true),
            error: None,
            instance: None,
        },
    )
    .unwrap();
}

#[test]
fn deactivation_error_redacts_echoed_license_key() {
    let error = deactivation_from_response(
        "LS-SECRET-RAW-KEY",
        LicenseApiResponse {
            activated: None,
            valid: None,
            deactivated: Some(false),
            error: Some("Cannot deactivate LS-SECRET-RAW-KEY.".to_string()),
            instance: None,
        },
    )
    .unwrap_err();

    assert!(error.to_string().contains("[license key]"));
    assert!(!error.to_string().contains("LS-SECRET-RAW-KEY"));
}

#[tokio::test]
async fn validate_posts_license_key_and_instance_id() {
    let (base_url, request) = capture_one_request(r#"{"valid":true}"#).await;
    let provider = LemonSqueezyProvider {
        client: LicenseApiClient::test(base_url),
    };

    assert!(provider
        .validate("LS-SECRET-RAW-KEY", "instance-1")
        .await
        .unwrap());

    let request = request.await.unwrap();
    assert!(request.starts_with("POST /validate HTTP/1.1"));
    assert!(request.contains("license_key=LS-SECRET-RAW-KEY&instance_id=instance-1"));
}

#[tokio::test]
async fn deactivate_posts_license_key_and_instance_id() {
    let (base_url, request) = capture_one_request(r#"{"deactivated":true}"#).await;
    let provider = LemonSqueezyProvider {
        client: LicenseApiClient::test(base_url),
    };

    provider
        .deactivate("LS-SECRET-RAW-KEY", "instance-1")
        .await
        .unwrap();

    let request = request.await.unwrap();
    assert!(request.starts_with("POST /deactivate HTTP/1.1"));
    assert!(request.contains("license_key=LS-SECRET-RAW-KEY&instance_id=instance-1"));
}

async fn capture_one_request(
    response_json: &'static str,
) -> (String, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert_ne!(read, 0);
            request.extend_from_slice(&buffer[..read]);
            if request_is_complete(&request) {
                break;
            }
        }
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{}",
            response_json.len(),
            response_json
        );
        stream.write_all(response.as_bytes()).await.unwrap();
        String::from_utf8(request).unwrap()
    });
    (base_url, task)
}

fn request_is_complete(request: &[u8]) -> bool {
    let Some(header_end) = request.windows(4).position(|window| window == b"\r\n\r\n") else {
        return false;
    };
    let headers = String::from_utf8_lossy(&request[..header_end]).to_lowercase();
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_default();
    request.len() >= header_end + 4 + content_length
}
