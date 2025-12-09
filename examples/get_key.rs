use cryptolens::{
    CryptolensClient,
    GetKeyArguments,
    verify_license_signature,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env
    let token = std::env::var("CRYPTOLENS_TOKEN")?;
    let public_key = std::env::var("CRYPTOLENS_PUBLIC_KEY_XML")?;

    let product_id: u64 = std::env::var("CRYPTOLENS_PRODUCT_ID")?.parse()?;
    let key = std::env::var("CRYPTOLENS_KEY")?;

    // client
    let client = CryptolensClient::new(token);

    // args
    let args = GetKeyArguments {
        ProductId: product_id,
        Key: &key,
    };

    // call
    let license = client.get_key(args).await?;

    // verify signature
    let ok = verify_license_signature(&license, &public_key)?;
    println!("signature ok: {}", ok);

    // some useful output
    println!("ProductId: {}", license.ProductId);
    println!("Key: {:?}", license.Key);
    println!("Expires: {}", license.Expires);
    println!("AllowedMachines: {:?}", license.AllowedMachines);
    println!("ActivatedMachines: {}", license.ActivatedMachines.len());

    Ok(())
}
