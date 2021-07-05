use vulcan_salute::*;
use vulcan_salute::util::*;
use vulcan_salute::util::cpu::*;
use std::ops::Deref;


/// 默认生成结构的时候需要这样，开始
#[macro_use]
struct_names! {
struct HelloController {
    node: web::Controller
}
}

impl HelloController{
    pub fn get_controller_name(&self)->String{
        String::from(Self::struct_name())
     }
}
/// 默认生成结构的时候需要这样，结束
/// 

impl Deref for HelloController {
    type Target = web::Controller;

    fn deref(&self) -> & web::Controller {
          return &self.node;
     }        
}
fn main() {
    println!("Hello, world!{:#?}",get_os_info());
    println!("Hello, world!{:#?}",read_issue().unwrap().trim());
    println!("cpu:{}",get_cpu());

    //let mut bird = HelloController::deref().;
    let   sprite = HelloController{ node: web::Controller::new(String::from("name"))};
    println!("===={}====",sprite.get_controller_name());
    //let mut sprite_node: &mut web::Controller = &mut sprite;
    //sprite.node::get_controller_name();
}
