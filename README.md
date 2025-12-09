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

Access Token with permissions:

Activate

Get Key

Product Id

A License Key created under your product

RSA Public Key (XML format) for signature verification

Recommended env variables for development:

```bash
export CRYPTOLENS_TOKEN="your_access_token"
export CRYPTOLENS_PRODUCT_ID="your_product_id"
export CRYPTOLENS_KEY="your_license_key"
export CRYPTOLENS_MACHINE_CODE="machine-1"
export CRYPTOLENS_PUBLIC_KEY_XML='<RSAKeyValue><Modulus>...</Modulus><Exponent>...</Exponent></RSAKeyValue>'

