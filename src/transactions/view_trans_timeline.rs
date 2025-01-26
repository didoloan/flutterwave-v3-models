use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::fwcall::{FwCall, ToFwCall};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ViewTransTimelineReq {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewTransTimelineRes {
    pub status: String,
    pub message: String,
    pub data: Vec<TimelineItem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineItem {
    pub note: String,
    pub actor: String,
    pub object: String,
    pub action: String,
    pub context: String,
    pub created_at: String,
}

impl<'a> ToFwCall<'a> for ViewTransTimelineReq {
    type ApiRequest = Self;

    type ApiResponse = ViewTransTimelineRes;

    fn get_call(self) -> crate::fwcall::FwCall<'a, Self::ApiRequest, Self::ApiResponse> {
        FwCall::new(
            std::borrow::Cow::Owned(format!("/v3/transactions/{}/events", self.id)),
            reqwest::Method::GET,
            None
        )
    }
}
