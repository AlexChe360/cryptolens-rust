use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptolensError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("xml error: {0}")]
    Xml(#[from] serde_xml_rs::Error),

    #[error("base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("utf8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("crypto error")]
    Crypto,

    #[error("api error: {code} {message}")]
    Api { code: i64, message: String },

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("signature invalid")]
    BadSignature,
}

pub type Result<T> = std::result::Result<T, CryptolensError>;
