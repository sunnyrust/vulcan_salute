//!
//!  这个是脚手架
//!

#[macro_use]
extern crate clap;
extern crate libc;
use clap::{App, SubCommand,Arg};
extern crate git2;
use git2::{Config};
use std::{thread, time};
mod lycli;
/// flag用于参数
fn flag() -> lycli::VulcanProject {
    let mut result=lycli::VulcanProject{project_name:"".to_string(),project_edition: String::from("2018")};
    let s="Version:".to_owned()+&crate_version!().to_owned()+"  git:"+&crate_description!().to_owned();
    // println!("{:?}",s);
    let matches = App::new("Lychee")
        .version(&*s)
        .author(crate_authors!())
        .about(
            "辅助vulcan的脚手架，用于生成Rsut的Web程序。
        例子:lychee new project_name
        ",
        )
        .args_from_usage("-e, --edver=[EDVER] 'Please type edition，For example： 2018.'")
        .subcommand(
            SubCommand::with_name("new")
                .about("Crating a new Project")
                .version(crate_version!())
                .author("Sunny Region. <jinheking@gmail.com>")
                .arg(Arg::with_name("input")
                    .help("Sets the new project to use")
                    .required(true)
                    .index(1)
                )
        )
        .get_matches();

        let mut input = String::from("helloworld");
        if let Some(matches) = matches.subcommand_matches("new") {
             input = matches.value_of("input").unwrap_or("helloworld").to_string();
        }
        // println!("project name:{}",input);
        result.project_name=input;
        let mut ed_ver = String::from("2018");
        result.project_edition=ed_ver;
        let edver=matches.value_of("edver").unwrap_or("2018");
        
        if  edver=="2015" {
            ed_ver=String::from("2015")
        }else if   edver=="2018" {
            ed_ver=String::from("2018")
        }else if   edver=="2021" {
            ed_ver=String::from("2021")
        }else {
            println!("💖 Edition only 2015,2018,2021!");
            use std::process;
            process::exit(0x0100);
        }
        
        result.project_edition=ed_ver;
        // if let Some(matches) = matches.subcommand_matches("new") {
        //      input = matches.value_of("input").unwrap_or("helloworld").to_string();
        // }
        result
    }

fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let vp=flag();

    // if vp.project_name=="".to_string(){
    //     println!("💖Please input project name!");
    //     use std::process;
    //     process::exit(0x0100);
    // }

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
    let project_name=vp.project_name; 
    let authors=String::from("[\"")+&name_value+" <"+&email_value+">\"]";
    let  edition=vp.project_edition;   
    let cargo=lycli::Cargo{name:"name = \"".to_string()+&project_name+"\"",version:"version = \"0.1.0\"".to_string(),authors:"authors = ".to_string()+&authors,edition:"edition = \"".to_string()+&edition+"\""};
    //println!("{:#?}",cargo);

    
    let cargo_toml_path=project_name.to_string()+"/Cargo.toml";
    let dir_name=project_name.to_string()+"/src";
    lycli::mkdir(&dir_name);
    let millis = time::Duration::from_millis(300);
    thread::sleep(millis);
    let _result=lycli::create_caego_toml(cargo, cargo_toml_path);
    thread::sleep(millis);
    let main_path=project_name.to_string()+"/src/main.rs";
    let _result=lycli::create_main(main_path);
}