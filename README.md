# Cryptolens Client API for Rust

> Status: Beta

A lightweight Rust SDK for the Cryptolens / Devolens Web API.

This library provides a clean client for:
- **Activate**
- **GetKey**

It also supports **RSA signature verification** of signed responses using:
- `Sign=true`
- `SignMethod=1` (String Sign)
- RSA + SHA256

The codebase is structured to make it easy to add more API methods later.

---

## Features

- Activate a license key (**Activate**)
- Retrieve license details (**GetKey**)
- Verify signed responses via RSA signature (String Sign)
- Robust deserialization for potentially missing/null fields
- Clear and extensible internal structure

---

## Requirements

- Rust 1.70+ (recommended: latest stable)
- Network access to the Web API

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cryptolens = "0.0.1"

```
## If you want to run examples:

```toml
[dev-dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

```

## Configuration
You will need:
1. Access Token with permissions:
- Activate
- Get Key
2. Product Id
3. A License Key created under your product
4. RSA Public Key (XML format) for signature verification

Recommended env variables for development:

```bash
export CRYPTOLENS_TOKEN="your_access_token"
export CRYPTOLENS_PRODUCT_ID="your_product_id"
export CRYPTOLENS_KEY="your_license_key"
export CRYPTOLENS_MACHINE_CODE="machine-1"
export CRYPTOLENS_PUBLIC_KEY_XML='<RSAKeyValue><Modulus>...</Modulus><Exponent>...</Exponent></RSAKeyValue>'

```
## Usage
Activate

```rust
use cryptolens::{CryptolensClient, KeyActivateArguments, verify_license_signature};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("CRYPTOLENS_TOKEN")?;
    let product_id: u64 = std::env::var("CRYPTOLENS_PRODUCT_ID")?.parse()?;
    let key = std::env::var("CRYPTOLENS_KEY")?;
    let machine = std::env::var("CRYPTOLENS_MACHINE_CODE")?;
    let public_key = std::env::var("CRYPTOLENS_PUBLIC_KEY_XML")?;

    let client = CryptolensClient::new(token);

    let args = KeyActivateArguments {
        ProductId: product_id,
        Key: &key,
        MachineCode: &machine,
        ..Default::default()
    };

    let license = client.activate(args).await?;

    let ok = verify_license_signature(&license, &public_key)?;
    println!("signature ok: {}", ok);

    println!("ProductId: {}", license.ProductId);
    println!("Key: {:?}", license.Key);
    println!("Expires: {}", license.Expires);
    println!("AllowedMachines: {:?}", license.AllowedMachines);
    println!("ActivatedMachines: {}", license.ActivatedMachines.len());

    Ok(())
}

```

## GetKey

```rust
use cryptolens::{CryptolensClient, GetKeyArguments, verify_license_signature};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("CRYPTOLENS_TOKEN")?;
    let product_id: u64 = std::env::var("CRYPTOLENS_PRODUCT_ID")?.parse()?;
    let key = std::env::var("CRYPTOLENS_KEY")?;
    let public_key = std::env::var("CRYPTOLENS_PUBLIC_KEY_XML")?;

    let client = CryptolensClient::new(token);

    let args = GetKeyArguments {
        ProductId: product_id,
        Key: &key,
    };

    let license = client.get_key(args).await?;

    let ok = verify_license_signature(&license, &public_key)?;
    println!("signature ok: {}", ok);

    println!("ProductId: {}", license.ProductId);
    println!("Key: {:?}", license.Key);
    println!("Expires: {}", license.Expires);
    println!("AllowedMachines: {:?}", license.AllowedMachines);
    println!("ActivatedMachines: {}", license.ActivatedMachines.len());

    Ok(())
}

```

## Signature Verification

With:
-- Sign=true
-- SignMethod=1
the server returns:
-- licenseKey: Base64-encoded JSON payload of license data
-- signature: Base64-encoded RSA signature of the licenseKey bytes
This SDK verifies the signature using the RSA Public Key of the product.

## Examples
This repository includes runnable examples:

```bash
cargo run --example activate
cargo run --example get_key

```
## Status and Roadmap

As long as the library is at version `0.0.X`, we do not guarantee semantic versioning stability.

### Implemented

- [x] **Activate**
- [x] **GetKey**
- [x] **RSA signature verification** (`SignMethod=1`)
- [x] More resilient deserialization of license payload

### Planned before `0.1.0`

- [ ] Improve and standardize error types and messages
- [ ] Return structured server errors when activation fails
- [ ] Decide time strategy:
  - [ ] Keep integer timestamps by default
  - [ ] Optionally add a `chrono` feature
- [ ] Polish naming to be more Rust-idiomatic
- [ ] Add integration tests for Activate/GetKey
- [ ] Optional: feature flags for `async` / `blocking`

