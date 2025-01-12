use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::charge_res_data::ChargeResData;


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CapturePreAuthChargeReq {
    pub flw_ref: String,
    pub body: CapturePreAuthChargeReqBody
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CapturePreAuthChargeReqBody {
    pub amount: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CapturePreAuthChargeRes {
    pub status: String,
    pub message: String,
    pub data: ChargeResData
}
