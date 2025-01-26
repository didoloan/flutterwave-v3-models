use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;
use crate::common::payload::Payload;

#[derive(Debug)]
pub struct FwCall<'a, T: Serialize, R: Deserialize<'a>> {
    #[allow(unused)]
    pub path: Cow<'a, str>,
    #[allow(unused)]
    pub method: reqwest::Method,
    #[allow(unused)]
    pub payload: Option<Payload<T>>,
    _phantom: PhantomData<R>,
}

impl<'a, T: Serialize, R: Deserialize<'a>> FwCall<'a, T, R> {
    pub fn new(path: Cow<'a, str>, method: reqwest::Method, payload: Option<Payload<T>>) -> Self {
        Self {
            path,
            method,
            payload,
            _phantom: PhantomData,
        }
    }
}

pub trait ToFwCall<'a>
where
    Self: Deserialize<'a> + Serialize + validator::Validate,
    Self::ApiResponse: DeserializeOwned + Serialize,
    Self::ApiRequest: Deserialize<'a> + Serialize + validator::Validate,
{
    type ApiRequest;

    type ApiResponse;

    #[allow(unused)]
    fn get_call(self) -> FwCall<'a, Self::ApiRequest, Self::ApiResponse>;

    #[allow(unused)]
    fn to_call(
        self,
    ) -> Result<
        FwCall<'a, Self::ApiRequest, Self::ApiResponse>,
        crate::errors::FWaveError,
    > {
        self.validate()?;
        Ok(self.get_call())
    }
}