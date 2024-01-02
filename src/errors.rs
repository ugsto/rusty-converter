use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum CurrencyCodeError {
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),
}
