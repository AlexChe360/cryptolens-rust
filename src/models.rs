use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ActivateResponse {
    pub result: i64,
    #[serde(alias = "messsage")]
    pub message: Option<String>,
    pub licenseKey: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GetKeyResponse {
    pub result: i64,
    #[serde(alias = "messsage")]
    pub message: Option<String>,
    pub licenseKey: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SerdeLicenseKey {
    pub ProductId: u64,
    pub Id: Option<u64>,
    pub Key: Option<String>,
    pub Created: u64,
    pub Expires: u64,
    pub Period: u64,
    pub F1: bool,
    pub F2: bool,
    pub F3: bool,
    pub F4: bool,
    pub F5: bool,
    pub F6: bool,
    pub F7: bool,
    pub F8: bool,
    pub Notes: Option<String>,
    pub Block: bool,
    pub GlobalId: Option<u64>,
    pub Customer: Option<Customer>,
    pub ActivatedMachines: Vec<ActivationData>,
    #[serde(default, alias = "TrialActiovation")]
    pub TrialActiovation: bool,
    #[serde(default, alias = "MaxNoMachines")]
    pub MaxNoMachines: Option<u64>,
    #[serde(default)]
    pub AllowedMachines: Option<String>,
    #[serde(default)]
    pub DataObjects: Vec<DataObject>,
    pub SignDate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LicenseKey {
    pub ProductId: u64,
    pub Id: Option<u64>,
    pub Key: Option<String>,
    pub Created: u64,
    pub Expires: u64,
    pub Period: u64,
    pub F1: bool,
    pub F2: bool,
    pub F3: bool,
    pub F4: bool,
    pub F5: bool,
    pub F6: bool,
    pub F7: bool,
    pub F8: bool,
    pub Notes: Option<String>,
    pub Block: bool,
    pub GlobalId: Option<u64>,
    pub Customer: Option<Customer>,
    pub ActivatedMachines: Vec<ActivationData>,
    #[serde(default, alias = "TrialActiovation")]
    pub TrialActivation: bool,
    #[serde(default, alias = "MaxNoMachines")]
    pub MaxNoOfMachines: Option<u64>,
    pub AllowedMachines: Vec<String>,
    pub DataObjects: Vec<DataObject>,
    pub SignDate: u64,

    pub(crate) license_key_bytes: Vec<u8>,
    pub(crate) signature_bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Customer {
    pub Id: u64,
    pub Name: Option<String>,
    pub Email: Option<String>,
    pub CompanyName: Option<String>,
    pub Created: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ActivationData {
    pub Mid: String,
    pub IP: String,
    pub Time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DataObject {
    pub Id: u64,

    #[serde(default)]
    pub Name: String,

    #[serde(default)]
    pub StringValue: String,

    pub IntValue: u64,
}