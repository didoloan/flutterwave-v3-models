use crate::{
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
use crate::common::customer::CustomerData;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AchReq {
    pub amount: i32,
    pub currency: String,
    pub email: String,
    pub tx_ref: String,
    pub fullname: String,
    pub phone_number: String,
    pub client_ip: String,
    pub device_fingerprint: String,
    pub meta: HashMap<String, String>,
    pub redirect_url: String,
    pub country: String,
}

impl<'a> ToFwCall<'a> for AchReq {
    type ApiRequest = Self;

    type ApiResponse = AchRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/charges?type=ach_payment"),
            reqwest::Method::PUT,
            Some(Payload::Plain(self)),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchRes {
    pub status: String,
    pub message: String,
    pub data: AchResData,
    pub meta: AchResMeta
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchResMeta {
    pub authorization: AchResAuthorization
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchResAuthorization {
    pub mode: String,
    pub redirect: String,
    pub validate_instructions: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AchResData {
    pub id: i64,
    pub tx_ref: String,
    pub flw_ref: String,
    pub device_fingerprint: String,
    pub amount: i64,
    pub charged_amount: i64,
    pub app_fee: f64,
    pub merchant_fee: i64,
    pub processor_response: String,
    pub auth_model: String,
    pub auth_url: String,
    pub currency: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub payment_type: String,
    pub fraud_status: String,
    pub charge_type: String,
    pub created_at: String,
    pub account_id: i64,
    pub redirect_url: String,
    pub customer: CustomerData,
}