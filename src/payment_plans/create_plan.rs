use super::{Interval, PlanApiRes};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::borrow::Cow;
use crate::{
    api_responses::ResponseType,
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlanReq {
    pub amount: i32,
    pub name: String,
    pub interval: Interval,
    pub duration: Option<i32>,
}

impl<'a> ToFwCall<'a> for CreatePlanReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/payment-plans"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}
