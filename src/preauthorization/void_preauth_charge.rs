use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::charge_res_data::ChargeResData;

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