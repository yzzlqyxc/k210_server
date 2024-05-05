#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod net;
mod tools;
mod screen;
extern crate lazy_static;
use core::arch::global_asm;
use core::include_str;
use k210_soc::spi::SPIExt;
use k210_soc::dmac::{dma_channel, DMACExt};
use k210_soc::sysctl;
use crate::net::connection::try_receive_remote;
use crate::tools::timer::sleep;
use screen::lcd::{Lcd, LCD_X_MAX, LCD_Y_MAX};

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
    let p = unsafe {k210_hal::pac::Peripherals::steal()};
    let dmc: k210_soc::dmac::DMAC = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    sysctl::set_spi0_dvp_data(true);
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
    let mut lcd = Lcd::new(spi, &dmc, dma_channel::CHANNEL0);

    lcd.on();
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
            tools::parse(&buf);
        } else {
            println!("timeout!");
        }
        sleep(20);
    }
    panic!("Shutdown machine!");
}