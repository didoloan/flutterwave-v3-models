pub mod custom_stat_code;
pub use custom_stat_code::StatCode;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseType<T> {
    Success(T),
    Error(FwErrorRes)
}

impl<T:> ResponseType<T> {
    pub fn replace_stat_code(self, status_code: StatusCode) -> Self {
        match self {
            ResponseType::Success(data) => ResponseType::Success(data),
            ResponseType::Error(mut err) => {
                err.status_code = status_code.into();
                ResponseType::Error(err)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FwErrorRes {
    #[serde(default)]
    pub status_code: StatCode,
    pub status: String,
    pub message: String,
    pub data: Option<ErrorResData>
}
