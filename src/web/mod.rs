use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pager { 
    pub pages: u32,
    pub current: u32,
    pub rows_total: u32,
    pub limit: u32,
}

#[allow(dead_code)]
/// 这个是Controller的基类
pub struct Controller{
    name:String, 
}

impl Controller {
    pub fn new(name: String) -> Controller {
        Controller { name: name }
    }
    pub  fn get_controller_name(&mut self)->String {
        String::from(&self.name)
   }
}
// trait GetControllerName {
//     /// 取得Controller的名字
//     fn get_controller_name(&mut self)->String;
// }

// impl GetControllerName for Controller {
//     fn get_controller_name(&mut self)->String {
//          String::from(&self.name)
//     }
// }
// pub trait Controller {
//     /// 处理额外的追回数据
//     fn index_after(_data: &mut tera::Context);

//     /// 处理编辑时需要展现出来的附加数据
//     fn edit_after(_data: &mut tera::Context);
//     /// 保存之后处理
//     fn save_after() ;

//     /// 删除之后处理
//     fn delete_after() ;
// }

pub mod template;
pub mod web_error;
