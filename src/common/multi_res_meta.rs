use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiResMeta {
    pub page_info: MultiResMetaPageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiResMetaPageInfo {
    pub total: i32,
    pub current_page: i32,
    pub total_pages: i32,
}
