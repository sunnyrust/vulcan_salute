use std::fs;

pub fn mkdir(path:&str){
    let r = fs::create_dir_all(path);
    match r {
        Err(e) => {
            println!("error creating {}: {}", path, e);
            std::process::exit(1);
        }
        Ok(_) => println!("created {}: OK", path),
    }
}
