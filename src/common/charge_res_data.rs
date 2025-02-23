use serde::{Deserialize, Serialize};
use crate::common::card_data_res::ResCardData;
use super::customer::CustomerData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeResData {
    pub id: i64,
    pub tx_ref: String,
    pub flw_ref: String,
    pub device_fingerprint: String,
    pub amount: i64,
    pub charged_amount: i64,
    pub app_fee: f64,
    pub merchant_fee: i64,
    pub processor_response: String,
    pub auth_model: String,
    pub currency: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub auth_url: String,
    pub payment_type: String,
    pub fraud_status: String,
    pub charge_type: String,
    pub created_at: String,
    pub account_id: i64,
    pub customer: CustomerData,
    pub card: ResCardData,
}
