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

use crate::net::connection::{print_from_wifi, try_receive_remote};
use crate::tools::timer::{get_time_ms, sleep};
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

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    net::test();
    println!("_______ ALL WORKS WELL _______"); 

    loop {
        let mut buf = [0u8;1024];
        let t = try_receive_remote(&mut buf);
        if let Ok((port, cnt)) = t {
            println!("{} {}", port, cnt);
            for i in 0..cnt {
                print!("{}", buf[i] as char);
            }
            print!("\n");
        } else {
            println!("timeout!");
        }
        sleep(20);
    }
    panic!("Shutdown machine!");
}