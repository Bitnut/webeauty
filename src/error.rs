#[cfg(feature = "cli")]
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use url::ParseError as UrlParseError;

#[derive(Error, Debug)]
pub enum Error {

    #[error("JSON Serde Error {0}")]
    SerdeJsonError(#[from] SerdeJsonError),

    #[error("URL Parse Error {0}")]
    UrlParseError(#[from] UrlParseError),

    #[cfg(feature = "cli")]
    #[error("Reqwest Error {0}")]
    ReqwestError(#[from] ReqwestError),
}
