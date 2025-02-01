use serde::{Deserialize, Serialize};
use validator::Validate;
use std::borrow::Cow;
use crate::fwcall::{FwCall, ToFwCall};
use super::PlanApiRes;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetPlanReq {
    pub id: i32,
}

impl<'a> ToFwCall<'a> for GetPlanReq {
    type ApiRequest = Self;

    type ApiResponse = PlanApiRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}", self.id)),
            reqwest::Method::GET,
            None,
        )
    }
}
