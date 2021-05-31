use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pager { 
    pub pages: u32,
    pub current: u32,
    pub rows_total: u32,
    pub limit: u32,
}


pub mod template;
pub mod web_error;
pub mod controller;