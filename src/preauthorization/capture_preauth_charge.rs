use serde::{Deserialize, Serialize};
use validator::Validate;

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
