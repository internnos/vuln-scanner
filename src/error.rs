use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
    #[error("Reqwest Error")]
    ReqwestError(#[from] reqwest::Error)
}