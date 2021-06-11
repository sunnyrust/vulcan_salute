//!
//!  这个是脚手架
//!

#[macro_use]
extern crate clap;
extern crate libc;
use clap::{App, SubCommand};
extern crate git2;
use git2::{Config};
use std::{thread, time};
mod lycli;
/// flag用于参数
fn flag() -> () {
    let s="Version:".to_owned()+&crate_version!().to_owned()+"  git:"+&crate_description!().to_owned();
    // println!("{:?}",s);
    let matches = App::new("Sunnycat")
        .version(&*s)
        .author(crate_authors!())
        .about(
            "日志检索，增强版cat.
        例子:sunnycat --keyword example --file log.txt
                 ./sunnycat lines -r 5,10
        ",
        )
        .args_from_usage("-k, --keyword=[KEYWORD] '搜索关键字'")
        .args_from_usage("-b, --bytekeyword=[BYTEKEYWORD] '搜索byte关键字'")
        .args_from_usage("-s,--str=[STRING]'转成中文'")
        .args_from_usage("-f ,--file=[FILE] 'Sets the input file to use'")
        .subcommand(
            App::new("lineonly")
                .about("只显示有关键字存在的行号")
                .version(crate_version!())
                .args_from_usage("-l, --list '只显示能够查找到关键字的行号。'" ,)
        )
        .subcommand(
            SubCommand::with_name("lines")
                .about("选择哪些行显示")
                .version(crate_version!())
                .author("Sunny Region. <jinheking@gmail.com>")
                .args_from_usage(
                    "-r --rows '输入行数，例如：-r 1,10,表示从第一行到第10行。'
                                    [LINES]  ' 1,10,表示从第一行到第10行。'",
                ),
        )
        .get_matches();
    }

fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    flag();


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
    let project_name="sunny";  //TODO 接收项目名称
    let authors=String::from("[\"")+&name_value+" <"+&email_value+">\"]";
    let  edition="2018";   //TODO 接收edition 版本
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