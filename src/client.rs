use crate::errors::Result;
use crate::endpoints::{KeyActivateArguments, GetKeyArguments};
use crate::utils::{license_from_activate, license_from_get_key};
use crate::models::LicenseKey;

pub struct CryptolensClient {
    http: reqwest::Client,
    token: String,
    base_url: String,
}

impl CryptolensClient {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token: token.into(),
            base_url: "https://app.cryptolens.io/api".to_string(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    #[allow(non_snake_case)]
    pub async fn activate(&self, args: KeyActivateArguments<'_, '_>) -> Result<LicenseKey> {
        let url = format!("{}/key/Activate", self.base_url);

        let params = [
            ("token", self.token.as_str()),
            ("ProductId", &args.ProductId.to_string()),
            ("Key", args.Key),
            ("MachineCode", args.MachineCode),
            ("FieldsToReturn", &args.FieldsToReturn.to_string()),
            ("FloatingTimeInterval", &args.FloatingTimeInterval.to_string()),
            ("MaxOverdraft", &args.MaxOverdraft.to_string()),
            ("Sign", "true"),
            ("SignMethod", "1"),
            ("v", "1"),
        ];

        let text = self.http
            .post(url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        license_from_activate(&text)
    }

    #[allow(non_snake_case)]
    pub async fn get_key(&self, args: GetKeyArguments<'_>) -> Result<LicenseKey> {
        let url = format!("{}/key/GetKey", self.base_url);

        let params = [
            ("token", self.token.as_str()),
            ("ProductId", &args.ProductId.to_string()),
            ("Key", args.Key),
            ("Sign", "true"),
            ("SignMethod", "1"),
            ("v", "1"),
        ];

        let text = self.http
            .post(url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        license_from_get_key(&text)
    }
}
