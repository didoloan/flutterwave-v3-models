use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerData {
    pub id: i64,
    pub phone_number: Option<String>,
    pub name: String,
    pub email: String,
    pub created_at: String
}