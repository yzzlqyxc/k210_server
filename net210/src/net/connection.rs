use k210_hal::time::Bps;
use k210_hal::prelude::*;
use nb::block;
use k210_soc::{sysctl, fpioa, gpiohs, gpio};
use k210_soc::fpioa::{io, function};

pub fn sent() {
    let ph = unsafe{k210_pac::Peripherals::steal()};

    let clocks = k210_hal::clock::Clocks::new();

    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_RX, function::UART1_TX);
    fpioa::set_function(io::WIFI_TX, function::UART1_RX);

    fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
    fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
    gpiohs::set_pin(8, true);
    gpiohs::set_direction(8, gpio::direction::OUTPUT);

    let wifi_s = ph.UART1.configure(Bps(115200), &clocks);
    let (mut tx, mut rx) = wifi_s.split();

    let t = "AT+CWJAP_DEF=\"CMCC-2.4G-313\",\"18788187147\"\r\n";
    for i in t.chars() {
        let respon = block!(tx.try_write(i as u8));
        respon.unwrap();
    }
    // sleep::usleep(200000);

    // let t = "AT+PING=\"www.baidu.com\"\r\n";
    // for i in t.chars() {
    //     let respon = block!(tx.try_write(i as u8));
    //     respon.unwrap();
    // }
    // let t = "AT+CIPSTAMAC_CUR?\r\n";
    // for i in t.chars() {
    //     let respon = block!(tx.try_write(i as u8));
    //     respon.unwrap();
    // }
    // loop {
    //     let t = block!(rx.try_read());
    //     if let Ok(ch) = t {
    //         print!("{}", ch as char);
    //     } else {
    //         break;
    //     }
    // }
}
