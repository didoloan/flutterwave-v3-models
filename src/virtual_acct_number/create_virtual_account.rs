use serde::{Deserialize, Serialize};
use crate::{api_responses::ResponseType, common::payload::Payload, fwcall::{FwCall, ToFwCall}};
use std::borrow::Cow;
use super::virt_res_acct_data::VirtualAcctResAcctData;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctCreationReq {
    #[validate(email)]
    pub email: String,
    pub bvn: String,
    pub amount: Option<i32>,
    pub tx_ref: Option<String>,
    pub is_permanent: Option<bool>,
    pub narration: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctCreationRes {
    pub status: String,
    pub message: String,
    pub data: VirtualAcctResAcctData
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VirtualAcctBulkCreationReq {
    pub accounts: i32,
    #[validate(email)]
    pub email: String,
    pub is_permanent: bool,
    pub frequency: Option<i32>,
    pub tx_ref: Option<String>,
    pub amount: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualAcctBulkCreationRes {
    pub status: String,
    pub message: String,
    pub data: VirtualAcctBulkCreateResData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualAcctBulkCreateResData {
    pub batch_id: String,
    pub response_code: String,
    pub response_message: String
}

impl<'a> ToFwCall<'a> for VirtualAcctCreationReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VirtualAcctCreationRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/virtual-account-numbers"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}

impl<'a> ToFwCall<'a> for VirtualAcctBulkCreationReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<VirtualAcctBulkCreationRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Borrowed("/v3/bulk-virtual-account-numbers"),
            reqwest::Method::POST,
            Some(Payload::Plain(self)),
        )
    }
}