use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    common::{charge_res_data::ChargeResData, payload::Payload},
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;

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

impl<'a> ToFwCall<'a> for VoidPreAuthChargeReq {
    type ApiRequest = Self;

    type ApiResponse = VoidPreAuthChargeRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/charges/{}/void", self.flw_ref)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}