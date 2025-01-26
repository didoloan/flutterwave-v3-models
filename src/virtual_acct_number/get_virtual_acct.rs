use serde::{Deserialize, Serialize};
use crate::fwcall::{FwCall, ToFwCall};

use super::virt_res_acct_data::VirtualAcctResAcctData;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetVirtualAccountReq {
    pub acct_no: String
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetVirtualAccountRes {
    pub status: String,
    pub message: String,
    pub data: VirtualAcctResAcctData
}


impl<'a> ToFwCall<'a> for GetVirtualAccountReq {
    type ApiRequest = Self;

    type ApiResponse = GetVirtualAccountRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(std::borrow::Cow::Owned(format!("/v3/virtual-account-numbers/{}", self.acct_no)),
            reqwest::Method::GET,
            None
        )
    }
}