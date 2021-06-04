//!
//!  这个是脚手架
//!
extern crate git2;
use git2::{Config};

mod lycli;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cargo { 
     name : String,
     version :String,
     authors : String,
     edition : String,
}

fn main() {
    let cfg = Config::open_default().unwrap();
    let mut name_value:String="".to_string();
    let mut email_value:String="".to_string();
    for entry in &cfg.entries(None).unwrap() {
       let entry = entry.unwrap();
       let name=entry.name().unwrap();
       if name=="user.name".to_string(){
        // println!("{} => {}", name, entry.value().unwrap());
        name_value=entry.value().unwrap().to_string();
       }else if  name =="user.email".to_string(){
        // println!("{} => {}", name, entry.value().unwrap());
        email_value=entry.value().unwrap().to_string();
       }
    }
    let mut authors=String::from("[")+&name_value+" <"+&email_value+">]";

    let cargo=Cargo{name:"name=".to_string()+&name_value,version:"0.1.0".to_string(),authors:authors,edition:"2018".to_string()};
    println!("{:#?}",cargo);

    lycli::mkdir("./hello/src");
}