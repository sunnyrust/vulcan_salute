use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::{ OpenOptions};
use std::io::{Write};
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Cargo { 
     pub name : String,
     pub version :String,
     pub authors : String,
     pub edition : String,
}
/// 创建目录
pub(crate)  fn mkdir(path:&str){
    let r = fs::create_dir_all(path);
    match r {
        Err(e) => {
            println!("error creating {}: {}", path, e);
            std::process::exit(1);
        }
        Ok(_) => println!("Project folder created {}: 👌", path),
    }
}

pub(crate) fn create_main(path:String)-> std::io::Result<()>{
    let mut buffer = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path).unwrap();
    let str_main=r#"
fn main(){
    println!("Hello vulcan salute!");
    println!("Live long and prosper!\nPeace and long life");
}
"#;
    buffer.write(str_main.as_bytes())?;
    Ok(())
}

/// 创建Cargo.toml文件
pub(crate) fn create_caego_toml(cargo: crate::lycli::Cargo,path:String)-> std::io::Result<()>{
    //println!("{}---{:#?}",path,cargo);
    let mut buffer = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path).unwrap();

    buffer.write(b"[package]")?;
    buffer.write(b"\n")?;
    buffer.write(cargo.name.trim().as_bytes())?;
    buffer.write(b"\n")?;
    buffer.write(cargo.version.trim().as_bytes())?;
    buffer.write(b"\n")?;

    buffer.write(cargo.authors.trim().as_bytes())?;
    buffer.write(b"\n")?;
    buffer.write(cargo.edition.trim().as_bytes())?;
    buffer.write(b"\n\n")?;

    buffer.write(r"# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html".as_bytes())?;
    buffer.write(b"\n\n")?;
    buffer.write(b"[dependencies]")?;
    buffer.write(b"\n")?;

    println!("Cargo.toml created.👌");
    buffer.flush()?;

    Ok(())
}