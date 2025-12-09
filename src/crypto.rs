use crate::errors::{Result, CryptolensError};
use crate::models::LicenseKey;
use serde::Deserialize;
use base64::Engine;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct RSAKeyValue {
    Modulus: String,
    Exponent: String,
}

pub fn verify_license_signature(license: &LicenseKey, public_key_xml: &str) -> Result<bool> {
    let public_key: RSAKeyValue = serde_xml_rs::from_str(public_key_xml)?;

    let modulus_bytes = base64::engine::general_purpose::STANDARD.decode(public_key.Modulus)?;
    let exponent_bytes = base64::engine::general_purpose::STANDARD.decode(public_key.Exponent)?;

    let modulus = openssl::bn::BigNum::from_slice(&modulus_bytes).map_err(|_| CryptolensError::Crypto)?;
    let exponent = openssl::bn::BigNum::from_slice(&exponent_bytes).map_err(|_| CryptolensError::Crypto)?;

    let rsa = openssl::rsa::Rsa::from_public_components(modulus, exponent).map_err(|_| CryptolensError::Crypto)?;
    let pkey = openssl::pkey::PKey::from_rsa(rsa).map_err(|_| CryptolensError::Crypto)?;

    let mut verifier = openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey)
        .map_err(|_| CryptolensError::Crypto)?;

    verifier.update(&license.license_key_bytes).map_err(|_| CryptolensError::Crypto)?;
    let ok = verifier.verify(&license.signature_bytes).map_err(|_| CryptolensError::Crypto)?;

    Ok(ok)
}
