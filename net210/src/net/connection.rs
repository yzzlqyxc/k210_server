use k210_hal::serial::{Rx, Tx};
use k210_hal::time::Bps;
use k210_hal::prelude::*;
use k210_pac::{Peripherals, UART1};
use nb::block;
use k210_soc::{sysctl, fpioa, gpiohs, gpio};
use k210_soc::fpioa::{io, function};

#[derive(PartialEq, Debug)]
pub enum NetError {
    Fail,
    Error,
    NotEnd 
}

#[derive(PartialEq, Debug)]
pub enum NetOk {
    UdpOk,
    NoError
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref ph : Peripherals = unsafe{k210_pac::Peripherals::steal()};
}

unsafe impl Sync for Peripherals{}


pub fn print_from_wifi(rx : &mut Rx<UART1>) -> Result<NetOk, NetError>{
    let mut cnt = 0;
    let mut s : u8 = 0;
    loop {
        cnt += 1;
        let t = block!(rx.try_read());
        if let Ok(ch) = t {
            if ch == 10 {
                break;
            } 
            print!("{}", ch as char);
            if cnt == 1 {
                s = ch;
            }
        } else {
            break;
        }

    }
    print!("\n");
    if cnt == 4 && s as char == 'O' {
        Ok(NetOk::NoError)
    } else if cnt == 6 && s as char == 'F' {
        Err(NetError::Fail)
    } else if cnt == 7 && s as char == 'E' {
        Err(NetError::Error)
    } else if cnt == 9 && s as char == 'S' {
        Ok(NetOk::UdpOk)
    } else {
        Err(NetError::NotEnd)
    }
}

pub fn at_command(t : &str, tx : &mut Tx<UART1>, rx : &mut Rx<UART1>) -> Result<NetOk, NetError> {
    for i in t.chars() {
        let respon = block!(tx.try_write(i as u8));
        respon.unwrap();
    }
    let mut t = print_from_wifi(rx);
    loop {
        t = print_from_wifi(rx);
        if t != Err(NetError::NotEnd) { 
            break;
        }
    }
    t
}

pub fn udp_send(id : u8, text : &str, tx: &mut Tx<UART1>, rx: &mut
            Rx<UART1>) -> Result<NetOk, NetError> {
    let command = "AT+CIPSEND=";

    for i in command.chars() {
        block!(tx.try_write(i as u8));
    }
    block!(tx.try_write(b'0'+ id));
    block!(tx.try_write(b','));
    let mut number = [0u8; 10];
    let mut lenth = text.len();
    let mut idx = 0;
    while lenth > 0 {
        number[idx] = (lenth % 10) as u8 + b'0';
        idx += 1; lenth /= 10;
    }    
    for &num in number.iter().rev() {
        if num != 0 {
            block!(tx.try_write(num));
        }
    }
    block!(tx.try_write(b'\r'));
    block!(tx.try_write(b'\n'));
    let mut t = print_from_wifi(rx);
    loop {
        t = print_from_wifi(rx);
        if t != Err(NetError::NotEnd) { 
            break;
        }
    }
    
    at_command(text, tx, rx)
}


    
pub fn sent() -> (Tx<UART1>, Rx<UART1>) {
    let clocks = k210_hal::clock::Clocks::new();

    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_RX, function::UART1_TX);
    fpioa::set_function(io::WIFI_TX, function::UART1_RX);

    fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
    fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
    gpiohs::set_pin(8, true);
    gpiohs::set_direction(8, gpio::direction::OUTPUT);

    let wifi_s = *ph.UART1.configure(Bps(115200), &clocks);
    let (mut tx, mut rx) = wifi_s.split();


    at_command("AT+CWLAP\r\n", &mut tx, &mut rx);
    at_command("AT+CWJAP_CUR=\"test\",\"12344321\"\r\n", &mut tx, &mut rx);
    at_command("AT+PING=\"47.93.124.97\"\r\n", &mut tx, &mut rx);
    (tx, rx)
}