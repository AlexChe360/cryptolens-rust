use crate::errors::{Result, CryptolensError};
use crate::models::{ActivateResponse, GetKeyResponse, LicenseKey, SerdeLicenseKey};
use base64::Engine;

const B64: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::general_purpose::STANDARD;

#[allow(non_snake_case)]
fn parse_license_common(
    result: i64,
    message: Option<String>,
    licenseKey: Option<String>,
    signature: Option<String>,
) -> Result<LicenseKey> {
    if result != 0 {
        return Err(CryptolensError::Api {
            code: result,
            message: message.unwrap_or_else(|| "Unknown error".to_string()),
        });
    }

    let license_key_b64 = licenseKey.ok_or_else(|| CryptolensError::InvalidResponse("licenseKey missing".into()))?;
    let signature_b64 = signature.ok_or_else(|| CryptolensError::InvalidResponse("signature missing".into()))?;

    let license_key_bytes = B64.decode(license_key_b64)?;
    let license_key_string = String::from_utf8(license_key_bytes.clone())?;
    let signature_bytes = B64.decode(signature_b64)?;

    let serde_lk: SerdeLicenseKey = serde_json::from_str(&license_key_string)?;

    let allowed_raw = serde_lk.AllowedMachines.unwrap_or_default();

    let allowed = allowed_raw
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(LicenseKey {
        ProductId: serde_lk.ProductId,
        Id: serde_lk.Id,
        Key: serde_lk.Key,
        Created: serde_lk.Created,
        Expires: serde_lk.Expires,
        Period: serde_lk.Period,
        F1: serde_lk.F1,
        F2: serde_lk.F2,
        F3: serde_lk.F3,
        F4: serde_lk.F4,
        F5: serde_lk.F5,
        F6: serde_lk.F6,
        F7: serde_lk.F7,
        F8: serde_lk.F8,
        Notes: serde_lk.Notes,
        Block: serde_lk.Block,
        GlobalId: serde_lk.GlobalId,
        Customer: serde_lk.Customer,
        ActivatedMachines: serde_lk.ActivatedMachines,
        TrialActivation: serde_lk.TrialActiovation,
        MaxNoOfMachines: serde_lk.MaxNoMachines,
        AllowedMachines: allowed,
        DataObjects: serde_lk.DataObjects,
        SignDate: serde_lk.SignDate,

        license_key_bytes,
        signature_bytes,
    })
}

pub fn license_from_activate(s: &str) -> Result<LicenseKey> {
    let r: ActivateResponse = serde_json::from_str(s)?;
    parse_license_common(r.result, r.message, r.licenseKey, r.signature)
}

pub fn license_from_get_key(s: &str) -> Result<LicenseKey> {
    let r: GetKeyResponse = serde_json::from_str(s)?;
    parse_license_common(r.result, r.message, r.licenseKey, r.signature)
}
