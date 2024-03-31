use core::cell::{RefCell, RefMut};
use core::ops::{Deref, DerefMut};

use crate::console::print;
use crate::tools::timer::{get_time, get_time_ms};
use k210_hal::clock::Clocks;
use k210_hal::prelude::*;
use k210_hal::serial::{Rx, Tx};
use k210_hal::time::Bps;
use k210_pac::UART1;
use k210_soc::fpioa::{function, io};
use k210_soc::{fpioa, gpio, gpiohs, sysctl};
use nb::block;

#[derive(PartialEq, Debug)]
pub enum NetError {
    Fail,
    Error,
    NotEnd,
}
#[derive(Debug)]
pub struct TIMEOUT;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref AA: UPIntrFreeCell<Transmit> = unsafe { UPIntrFreeCell::new(Transmit::new()) };
}
#[derive(PartialEq, Debug)]
pub enum NetOk {
    UdpOk,
    NoError,
}
pub struct Transmit {
    clocks: Clocks,
    tx: Tx<UART1>,
    rx: Rx<UART1>,
    timeout: usize,
}
impl Transmit {
    pub fn new() -> Self {
        let clocks = k210_hal::clock::Clocks::new();
        let ph = unsafe { k210_pac::Peripherals::steal() };
        sysctl::clock_enable(sysctl::clock::UART1);
        sysctl::reset(sysctl::reset::UART1);
        fpioa::set_function(io::WIFI_RX, function::UART1_TX);
        fpioa::set_function(io::WIFI_TX, function::UART1_RX);

        fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
        fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
        gpiohs::set_pin(8, true);
        gpiohs::set_direction(8, gpio::direction::OUTPUT);
        let wifi_s = ph.UART1.configure(Bps(115200), &clocks);
        let (tx, rx) = wifi_s.split();
        Transmit {
            clocks,
            tx,
            rx,
            timeout: 20,
        }
    }
    pub fn sent(&mut self, t: &str) {
        for i in t.chars() {
            let _ = block!(self.tx.try_write(i as u8));
        }
    }
    pub fn sent_char(&mut self, t: u8) {
        let _ = block!(self.tx.try_write(t as u8));
    }
    pub fn get_char(&mut self) -> Result<u8, TIMEOUT>{
        let now = get_time_ms();
        while get_time_ms() - now <= self.timeout {
            if let Ok(u) = self.rx.try_read() {
                return Ok(u);
            }
        }
        // let t = self.rx.try_read();
        Err(TIMEOUT)
    }

    pub fn get_time(&self) -> u32 {
        self.clocks.cpu().0
    }
}

pub struct UPIntrFreeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPIntrFreeCell<T> {}
pub struct UPIntrRefMut<'a, T>(Option<RefMut<'a, T>>);

impl<T> UPIntrFreeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    pub fn ex(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
impl<'a, T> Deref for UPIntrRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap().deref()
    }
}
impl<'a, T> DerefMut for UPIntrRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap().deref_mut()
    }
}

pub fn print_from_wifi() -> Result<NetOk, NetError> {
    let mut cnt = 0;
    let mut s: u8 = 0;
    loop {
        // let ch = block!(rx.try_read()).unwrap();
        let re = AA.ex().get_char();
        // println!("{:?}", re);
        if let Ok(c) = re {
            cnt += 1;
            if c == 10 {
                break;
            }
            print!("{}", c as char);
            if cnt == 1 {
                s = c;
            }
        } else {
            continue;
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

pub fn at_command(t: &str) -> Result<NetOk, NetError> {
    AA.ex().sent(t);
    let mut t = print_from_wifi();
    loop {
        t = print_from_wifi();
        if t != Err(NetError::NotEnd) {
            break;
        }
    }
    t
}

pub fn udp_send(id: u8, text: &str) -> Result<NetOk, NetError> {
    let command = "AT+CIPSEND=";

    for i in command.chars() {
        AA.ex().sent_char(i as u8);
    }
    AA.ex().sent_char(b'0' + id);
    AA.ex().sent_char(b',');
    let mut number = [0u8; 10];
    let mut lenth = text.len();
    let mut idx = 0;
    while lenth > 0 {
        number[idx] = (lenth % 10) as u8 + b'0';
        idx += 1;
        lenth /= 10;
    }
    for &num in number.iter().rev() {
        if num != 0 {
            AA.ex().sent_char(num);
        }
    }
    AA.ex().sent_char(b'\r');
    AA.ex().sent_char(b'\n');
    let mut t = print_from_wifi();
    loop {
        t = print_from_wifi();
        if t != Err(NetError::NotEnd) {
            break;
        }
    }

    at_command(text)
}

fn get_num() -> usize {
    let mut num = 0;
    loop {
        let t = AA.ex().get_char(); 
        if let Ok(u) = t {
            if u >= b'0' && u <= b'9' {
                num = num * 10 + u as usize - '0' as usize;
            } else {
                break;
            }
        }
    }
    num
}

fn get_buf(buf: &mut [u8] , lenth : usize)  {
    println!("{}", lenth);
    for i in 0..lenth {
        loop {
            let r = AA.ex().get_char();
            if let Ok(u) = r {
                buf[i] = u;
                break;
            }
        }
    }
}

pub fn try_receive_remote(buf : &mut [u8]) -> Result<(usize, usize), TIMEOUT> {
    let mut cnt = 0;
    let mut port = 0;
    let mut lenth = 0;
    let now = get_time_ms();
    loop {
        if get_time_ms() - now >= 200 && cnt == 0{
            return Err(TIMEOUT);
        }
        let r : Result<u8, TIMEOUT> = AA.ex().get_char();
        if let Ok(u) = r {
            if u != 10 && u != 13 {
                cnt += 1;
            }
            
            if cnt == 5 {
                port = get_num();
                lenth = get_num();
                get_buf(buf, lenth);
                return Ok((port, lenth))
            } 
        }
        else{
            return Err(TIMEOUT)
        }

    }
}