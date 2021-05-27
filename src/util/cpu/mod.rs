#[cfg(target_arch = "x86_64")]
pub mod cpu_x86_64;
#[cfg(target_arch = "x86_64")]
pub use self:: cpu_x86_64 as cpu_son;

#[cfg(target_arch = "arm")]
pub mod cpu_arm;

#[cfg(target_arch = "arm")]
pub use self::cpu_arm as cpu_son;

#[cfg(target_arch = "aarch64")]
pub mod cpu_aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::cpu_aarch64 as cpu_son;


pub  fn get_cup() -> String{
    cpu_son::get_cup()
}