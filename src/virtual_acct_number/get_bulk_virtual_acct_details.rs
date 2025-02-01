use super::virt_res_acct_data::VirtualAcctResAcctData;
use crate::{
    common::payload::Payload,
    fwcall::{FwCall, ToFwCall},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BulkVirtualAcctDetailsReq {
    pub batch_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkVirtualAcctDetailsRes {
    pub status: String,
    pub message: String,
    pub data: Vec<VirtualAcctResAcctData>,
}

impl<'a> ToFwCall<'a> for BulkVirtualAcctDetailsReq {
    type ApiRequest = Self;

    type ApiResponse = BulkVirtualAcctDetailsRes;

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
