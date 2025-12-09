use cryptolens::{CryptolensClient, KeyActivateArguments, verify_license_signature};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("CRYPTOLENS_TOKEN")?;
    let public_key = std::env::var("CRYPTOLENS_PUBLIC_KEY_XML")?;

    let client = CryptolensClient::new(token);
    let product_id: u64 = std::env::var("CRYPTOLENS_PRODUCT_ID")?.parse()?;
    let key = std::env::var("CRYPTOLENS_KEY")?;

    let args = KeyActivateArguments {
        ProductId: product_id,
        Key: &key,
        MachineCode: "machine-1",
        ..Default::default()
    };

    let license = client.activate(args).await?;

    let ok = verify_license_signature(&license, &public_key)?;
    println!("signature ok: {}", ok);

    Ok(())
}
