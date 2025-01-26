use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseType<T> {
    Success(T),
    Error400(FwError400Res),
    Error500(FwError500Res),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResData {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FwError400Res {
    pub status: String,
    pub message: String,
    pub data: Option<ErrorResData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FwError500Res {
    pub status: String,
    pub message: String,
    pub data: Option<ErrorResData>,
}