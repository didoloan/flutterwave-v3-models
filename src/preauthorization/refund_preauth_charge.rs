use crate::common::charge_res_data::ChargeResData;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RefundPreAuthChargeReq {
    pub flw_ref: String,
    pub body: RefundPreAuthChargeReqBody
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefundPreAuthChargeReqBody {
    pub amount: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefundPreAuthChargeRes {
    pub status: String,
    pub message: String,
    pub data: ChargeResData
}