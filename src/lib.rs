pub mod util;
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
}
