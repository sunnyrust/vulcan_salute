use vulcan_salute::*;
use vulcan_salute::util::*;
use vulcan_salute::util::cpu::*;


fn main() {
    println!("Hello, world!{:#?}",get_os_info());
    println!("Hello, world!{:#?}",read_issue().unwrap().trim());
    println!("cpu:{}",get_cpu());
    
}
