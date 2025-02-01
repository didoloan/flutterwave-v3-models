use serde::{Deserialize, Serialize};
use validator::Validate;
use std::borrow::Cow;
use crate::{
    common::{charge_res_data::ChargeResData, payload::Payload},
    fwcall::{FwCall, ToFwCall},
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RefundTransactionReq {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundTransactionRes {
    pub status: String,
    pub message: String,
    pub data: ChargeResData
}


impl<'a> ToFwCall<'a> for RefundTransactionReq {
    type ApiRequest = Self;

    type ApiResponse = RefundTransactionRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/transactions/{}/refund", self.id)),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}