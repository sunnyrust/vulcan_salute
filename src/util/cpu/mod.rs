#[cfg(target_arch = "x86")]
pub mod cpu_aarch64;
#[cfg(target_arch = "x86")]
pub use self::cpu_x86 as cpu_son;
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


#[cfg(target_arch = "mips")]
pub mod cpu_mips;
#[cfg(target_arch = "mips")]
pub use self::cpu_mips as cpu_son;

#[cfg(target_arch = "mips64")]
pub mod cpu_mips64;
#[cfg(target_arch = "mips64")]
pub use self::cpu_mips64 as cpu_son;

#[cfg(target_arch = "powerpc")]
pub mod cpu_powerpc;
#[cfg(target_arch = "powerpc")]
pub use self::cpu_powerpc as cpu_son;

#[cfg(target_arch = "sparc")]
pub mod cpu_sparc;
#[cfg(target_arch = "sparc")]
pub use self::cpu_sparc as cpu_son;

pub  fn get_cup() -> String{
    cpu_son::get_cup()
}