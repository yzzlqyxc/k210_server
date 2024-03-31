#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod net;
mod tools;
extern crate lazy_static;
use core::arch::global_asm;
use core::include_str;

use crate::tools::timer::sleep;
global_asm!(include_str!("entry.asm"));


fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

use core::arch::asm;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    // net::test();
    let r : usize;
    unsafe {asm!("csrr {0}, mcounteren", out(reg) r);}
    println!("{}", r);
    
    println!("_______ ALL WORKS WELL _______"); 
    sleep(500);
    println!("_______ ALL WORKS WELL _______"); 

    panic!("Shutdown machine!");
}