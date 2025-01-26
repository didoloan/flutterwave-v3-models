use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    api_responses::ResponseType,
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct QueryTransFeesReq {
    pub amount: i32,
    pub currency: String,
    pub payment_type: String,
    pub card_first6digits: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTransFeesRes {
    pub status: String,
    pub message: String,
    pub data: QueryTransFeesResData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTransFeesResData {
    pub charge_amount: String,
    pub fee: i32,
    pub merchant_fee: i32,
    pub flutterwave_fee: i32,
    pub stamp_duty_fee: i32,
    pub currency: String
}

impl<'a> ToFwCall<'a> for QueryTransFeesReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<QueryTransFeesRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/transactions/fees")),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}
