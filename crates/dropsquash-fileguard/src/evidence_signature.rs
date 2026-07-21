use std::path::Path;

use dropsquash_core::{AppError, Result};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceSignature {
    pub algorithm: String,
    pub public_key_hex: String,
    pub signature_hex: String,
}

pub fn sign_evidence(payload: &[u8], key_path: &Path) -> Result<EvidenceSignature> {
    let key_pair = load_or_create_key_pair(key_path)?;
    let signature = key_pair.sign(payload);
    let evidence = EvidenceSignature {
        algorithm: "ed25519".to_string(),
        public_key_hex: hex(key_pair.public_key().as_ref()),
        signature_hex: hex(signature.as_ref()),
    };
    verify_evidence_signature(payload, &evidence)?;
    Ok(evidence)
}

pub fn verify_evidence_signature(payload: &[u8], evidence: &EvidenceSignature) -> Result<()> {
    if evidence.algorithm != "ed25519" {
        return Err(invalid("uses an unsupported algorithm"));
    }
    let public_key = decode_hex(&evidence.public_key_hex)?;
    let signature = decode_hex(&evidence.signature_hex)?;
    UnparsedPublicKey::new(&ED25519, public_key)
        .verify(payload, &signature)
        .map_err(|_| invalid("could not be verified"))
}

fn load_or_create_key_pair(path: &Path) -> Result<Ed25519KeyPair> {
    if path.is_file() {
        restrict_key_permissions(path)?;
        return load_key_pair(&std::fs::read(path)?);
    }
    let parent = path
        .parent()
        .ok_or_else(|| invalid("key path has no parent"))?;
    std::fs::create_dir_all(parent)?;
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new())
        .map_err(|_| invalid("key generation failed"))?;
    std::fs::write(path, pkcs8.as_ref())?;
    restrict_key_permissions(path)?;
    load_key_pair(pkcs8.as_ref())
}

fn load_key_pair(pkcs8: &[u8]) -> Result<Ed25519KeyPair> {
    Ed25519KeyPair::from_pkcs8(pkcs8).map_err(|_| invalid("key is invalid"))
}

#[cfg(unix)]
fn restrict_key_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn restrict_key_permissions(_: &Path) -> Result<()> {
    Ok(())
}

fn decode_hex(value: &str) -> Result<Vec<u8>> {
    if value.len() % 2 != 0 {
        return Err(invalid("contains invalid hex"));
    }
    (0..value.len())
        .step_by(2)
        .map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| invalid("contains invalid hex"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn invalid(reason: &str) -> AppError {
    AppError::InvalidConfig(format!("Secure Share evidence signature {reason}"))
}

#[cfg(test)]
mod tests;
