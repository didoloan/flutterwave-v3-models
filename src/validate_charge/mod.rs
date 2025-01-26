use crate::common::card_data_res::ResCardData;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    api_responses::ResponseType,
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};
use std::borrow::Cow;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ValidateChargeReq {
    #[validate(length(min = 6, max = 6))]
    pub otp: String,
    pub flw_ref: String,
    #[serde(rename = "type")]
    pub charge_type: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateChargeRes {
    pub status: String,
    pub message: String,
    pub data: ValidateChargeResData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateChargeResData {
    id: i32,
    tx_ref: String,
    flw_ref: String,
    device_fingerprint: String,
    amount: i32,
    charge_amount: i32,
    app_fee: i32,
    merchant_fee: i32,
    processor_response: String,
    auth_model: String,
    currency: String,
    ip: String,
    narration: String,
    status: String,
    auth_url: String,
    payment_type: String,
    fraud_status: String,
    charge_type: String,
    created_at: String,
    account_id: i32,
    customer: ResponseCustomerData,
    card: ResCardData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCustomerData {
    id: i32,
    phone_number: String,
    name: String,
    email: String,
    created_at: String,
}

impl<'a> ToFwCall<'a> for ValidateChargeReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<ValidateChargeRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/validate-charge"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}
