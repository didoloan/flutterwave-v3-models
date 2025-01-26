use super::{Interval, PlanApiResData};
use crate::common::multi_res_meta::MultiResMeta;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{
    api_responses::ResponseType,
    fwcall::{FwCall, ToFwCall},
};
use serde_qs;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetPlansReq {
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i32>,
    pub amount: Option<i32>,
    pub currency: Option<String>,
    pub interval: Option<Interval>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlansRes {
    pub status: String,
    pub message: String,
    pub meta: MultiResMeta,
    pub data: Vec<PlanApiResData>,
}

impl<'a> ToFwCall<'a> for GetPlansReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<GetPlansRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/payment-plans{}",
                serde_qs::to_string(&self).unwrap()
            )),
            reqwest::Method::GET,
            None,
        )
    }
}
