use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    common::{charge_res_data::ChargeResData, payload::Payload},
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;

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

impl<'a> ToFwCall<'a> for RefundPreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = RefundPreAuthChargeRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/refund", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}
