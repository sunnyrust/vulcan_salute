use vulcan_salute::*;
use vulcan_salute::util::*;
use vulcan_salute::util::cpu::*;
use std::ops::Deref;
struct HelloController {
    node: web::Controller
}

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
    let  mut sprite = HelloController{ node: web::Controller::new(String::from("name"))};
    sprite.node.get_controller_name();
    //let mut sprite_node: &mut web::Controller = &mut sprite;
    //sprite.node::get_controller_name();
}
