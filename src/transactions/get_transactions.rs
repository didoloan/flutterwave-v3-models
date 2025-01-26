use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{common::multi_res_meta::MultiResMeta, fwcall::{FwCall, ToFwCall}};
use super::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetTransactionsReq {
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i32>,
    pub customer_email: Option<String>,
    pub status: Option<String>,
    pub tx_ref: Option<String>,
    pub customer_fullname: Option<String>,
    pub currency: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionsRes {
    pub status: String,
    pub message: String,
    pub meta: MultiResMeta,
    pub data: Vec<Transaction>
}

impl<'a> ToFwCall<'a> for GetTransactionsReq {
    type ApiRequest = Self;

    type ApiResponse = GetTransactionsRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            std::borrow::Cow::Owned(format!("/v3/transactions?{}", serde_qs::to_string(&self).unwrap())),
            reqwest::Method::GET,
            None
        )
    }
}