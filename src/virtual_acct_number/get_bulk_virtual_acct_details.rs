use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{api_responses::ResponseType, common::payload::Payload, fwcall::{FwCall, ToFwCall}};
use super::virt_res_acct_data::VirtualAcctResAcctData;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BulkVirtualAcctDetailsReq {
    pub batch_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkVirtualAcctDetailsRes {
    pub status: String,
    pub message: String,
    pub data: Vec<VirtualAcctResAcctData>
}

impl<'a> ToFwCall<'a> for BulkVirtualAcctDetailsReq {
    type ApiRequest = Self;

    type ApiResponse = ResponseType<BulkVirtualAcctDetailsRes>;

    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            Cow::Owned(format!(
                "/v3/bulk-virtual-account-numbers/{}",
                self.batch_id
            )),
            reqwest::Method::GET,
            Some(Payload::Plain(self)),
        )
    }
}