use std::default::Default;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::fwcall::{FwCall, ToFwCall};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResendFailedWebhookReq {
    pub id: String,
    pub query: Option<ResendHookQuery>
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResendHookQuery {
    pub wait: i32
}

impl Default for ResendHookQuery {
    fn default() -> Self {
        Self { wait: 1 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResendFailedWebhookRes {
    pub status: String,
    pub message: String,
    pub data: Option<String>
}

impl<'a> ToFwCall<'a> for ResendFailedWebhookReq {
    type ApiRequest = Self;

    type ApiResponse = ResendFailedWebhookRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            std::borrow::Cow::Owned(format!("/v3/transactions/{}/resend-hook?{}", self.id, serde_qs::to_string(&self.query.unwrap_or_default()).unwrap())), 
            reqwest::Method::POST, 
            None
        )
    }
}