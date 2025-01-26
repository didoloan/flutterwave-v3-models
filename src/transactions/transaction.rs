use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub tx_ref: String,
    pub flw_ref: String,
    pub device_fingerprint: String,
    pub amount: i64,
    pub currency: String,
    pub charged_amount: i64,
    pub app_fee: Option<f64>, // Note: Using Option to handle null
    pub merchant_fee: i64,
    pub processor_response: Option<String>, // Note: Using Option to handle null
    pub auth_model: String,
    pub ip: String,
    pub narration: String,
    pub status: String,
    pub payment_type: String,
    pub created_at: String,
    pub amount_settled: Option<f64>, // Note: Using Option to handle null
    pub account: TransAccount,
    pub customer_name: String,
    pub customer_email: String,
    pub account_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransAccount {
    pub nuban: String,
    pub bank: String
}
