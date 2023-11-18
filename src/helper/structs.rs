use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Meta {
    total: u16,
    page: u16,
    limit: u16,
}
