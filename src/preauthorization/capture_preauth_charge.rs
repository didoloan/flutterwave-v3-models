use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    common::{charge_res_data::ChargeResData, payload::Payload},
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;


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

impl<'a> ToFwCall<'a> for CapturePreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = CapturePreAuthChargeRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/capture", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

