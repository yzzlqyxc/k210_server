use k210_hal::serial::{Rx, Tx};
use k210_hal::time::Bps;
use k210_hal::prelude::*;
use k210_pac::UART1;
use nb::block;
use k210_soc::{sysctl, fpioa, gpiohs, gpio};
use k210_soc::fpioa::{io, function};

fn print_from_wifi(rx : &mut Rx<UART1>) {
    loop {
        let t = block!(rx.try_read());
        if let Ok(ch) = t {
            if ch == 10 {
                print!("{} {}\n", ch as char, ch);
                break;
            } 
            print!("{} {}\n", ch as char, ch);
        } else {
            break;
        }
    }
    print!("\n");
}

fn at_command(t : &str, tx : &mut Tx<UART1>, rx : &mut Rx<UART1>) {
    for i in t.chars() {
        let respon = block!(tx.try_write(i as u8));
        respon.unwrap();
    }
    print_from_wifi(rx);
    print_from_wifi(rx);
    print_from_wifi(rx);
    print_from_wifi(rx);
    print_from_wifi(rx);
}

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

    // at_command("AT+CWJAP_DEF=\"test\",\"12344321\"\r\n", &mut tx, &mut rx);
    at_command("AT+PING=\"www.baidu.com\"\r\n", &mut tx, &mut rx);
}
