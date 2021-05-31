pub mod util;
pub mod web;
#[cfg(target_os = "linux")]
mod issue_linux;
#[cfg(target_os = "linux")]
use issue_linux as platform;

use std::error::Error;

pub  fn read_issue() -> Result<String, Box<dyn Error>>{
    platform::read_issue()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    use crate::util::*;
    #[test]
    fn it_get(){
       println!("{:?}", get_os_info());
    }
    use super::*;
    #[test]
    fn get_current(){
        assert_eq!("Ubuntu 18.04.5 LTS \\n \\l",read_issue().unwrap().trim());
    }
}
