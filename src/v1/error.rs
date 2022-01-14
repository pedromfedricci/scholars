use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq)]
#[serde(rename_all = "camelCase")]
#[error("{error}")]
pub struct ClientError {
    error: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub struct ServerError {
    message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq)]
#[serde(untagged)]
pub enum ResponseError {
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error(transparent)]
    Server(#[from] ServerError),
}
