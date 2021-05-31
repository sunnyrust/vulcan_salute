use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pager { 
    pub pages: u32,
    pub current: u32,
    pub rows_total: u32,
    pub limit: u32,
}


pub trait Controller {
    /// 处理额外的追回数据
    fn index_after(_data: &mut tera::Context);

    /// 处理编辑时需要展现出来的附加数据
    fn edit_after(_data: &mut tera::Context);
    /// 保存之后处理
    fn save_after() ;

    /// 删除之后处理
    fn delete_after() ;
}
pub mod template;
pub mod web_error;
