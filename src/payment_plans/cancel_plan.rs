use crate::fwcall::{FwCall, ToFwCall};
use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::payment_plans::PlanApiRes;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CancelPlanReq {
    pub plan_id: i32,
}

impl<'a> ToFwCall<'a> for CancelPlanReq {
    type ApiRequest = Self;

    type ApiResponse = PlanApiRes;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}/cancel", self.plan_id)),
            reqwest::Method::PUT,
            None,
        )
    }
}
