use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct VoidPreAuthChargeReq {
    pub flw_ref: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoidPreAuthChargeRes {
    pub status: String,
    pub message: String,
    pub data: ChargeResData
}