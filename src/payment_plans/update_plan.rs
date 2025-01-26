use std::borrow::Cow;

use crate::{
    api_responses::ResponseType,
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use super::PlanApiRes;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlanReq {
    pub id: i32,
    pub body: UpdatePlanReqBody
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct UpdatePlanReqBody {
    pub name: String,
    pub status: PlanStatus,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum PlanStatus {
    #[default]
    Active,
    Inactive,
}

impl<'a> ToFwCall<'a> for UpdatePlanReq {
    type ApiRequest = UpdatePlanReqBody;

    type ApiResponse = ResponseType<PlanApiRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/payment-plans/{}", self.id)),
            reqwest::Method::PUT,
            Some(Payload::Plain(self.body)),
        )
    }
}
