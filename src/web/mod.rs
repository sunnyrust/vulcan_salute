// use ::function_name::named;
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
#[allow(unused_macros)]
#[macro_export]
macro_rules! struct_names {
    (pub struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[allow(dead_code)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl  $name {
            const fn field_names() -> &'static[&'static str] {
                &[$(stringify!($fname)),*]
            }

            const fn struct_name() -> &'static str{
                stringify!($name)
            }
        }
    }
}


pub struct Controller{
    name:String, 
}

// #[macro_use] extern crate function_name;
// macro_rules! function_path {() => (concat!(
//     module_path!(), "::", function_name!()
// ))}

impl Controller {
    ///在 Rust 中，构造函数并不特殊——但按照惯例，它通常是一个名为 new 的静态方法。
    /// 但既然是约定俗成的东西，感觉就像是一个构造函数，但所有规则都可以微调，而且简单很多。
    // #[named]
    pub fn new(name:  String) -> Controller {
        // if name==String::from("name"){
        //     name= String::from(  module_path!());
        // }
        println!("Module===>{:?}",module_path!());
        Controller { name: name }
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
#[macro_use]struct_names! {
    pub struct VulcanController {
        node: Controller
    }
}

use std::ops::Deref;
impl Deref for VulcanController {
    type Target = Controller;

    fn deref(&self) -> & Controller {
          return &self.node;
     }                
}
pub mod template;
pub mod web_error;
