use cryptolens::{CryptolensClient, KeyActivateArguments, verify_license_signature};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("CRYPTOLENS_TOKEN")?;
    let public_key = std::env::var("CRYPTOLENS_PUBLIC_KEY_XML")?;

    let client = CryptolensClient::new(token);

    let args = KeyActivateArguments {
        ProductId: 31733,
        Key: "MJTGH-SVNKG-EXYTO-JHNTX",
        MachineCode: "machine-1",
        ..Default::default()
    };

    let license = client.activate(args).await?;

    let ok = verify_license_signature(&license, &public_key)?;
    println!("signature ok: {}", ok);

    Ok(())
}
