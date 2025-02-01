use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    common::multi_res_meta::MultiResMeta,
    fwcall::{FwCall, ToFwCall},
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FetchRefundedTransReq {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FetchMultiRefundedTransReq {
    pub from: Option<String>,
    pub to: Option<String>,
    pub status: Option<String>,
    pub currency: Option<String>,
    pub flw_ref: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FetchRefundedTransRes {
    pub id: i32,
    pub amount_refunded: i32,
    pub status: String,
    pub flw_ref: String,
    pub comment: Option<String>,
    pub settlement_id: String,
    pub meta: Meta,
    pub created_at: String,
    pub account_id: i32,
    pub transaction_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FetchMultiRefundedTransRes {
    pub status: String,
    pub message: String,
    pub meta: MultiResMeta,
    pub data: Vec<FetchRefundedTransRes>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Meta {
    source: String,
    uniquereference: String,
    provider: String,
}

impl<'a> ToFwCall<'a> for FetchRefundedTransReq {
    type ApiRequest = Self;

    type ApiResponse = FetchRefundedTransRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!("/v3/refunds/{}", self.id)),
            reqwest::Method::GET,
            None,
        )
    }
}

impl<'a> ToFwCall<'a> for FetchMultiRefundedTransReq {
    type ApiRequest = Self;

    type ApiResponse = FetchMultiRefundedTransRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/refunds?{}",
                serde_qs::to_string(&self).unwrap()
            )),
            reqwest::Method::GET,
            None,
        )
    }
}
