use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Meta {
    total: u16,
    page: u16,
    limit: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct APIError {
    message: String,
    code: u32,
}

impl APIError {
    pub fn is_empty(&self) -> bool {
        self.message.is_empty()
    }
}
