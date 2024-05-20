use crate::{net::connection::{udp_send, AA}, screen::lcd::Lcd};
use k210_soc::spi::SPI;

pub mod timer;

pub fn strcmp(a : &[u8], b : &str) -> bool {
    let t = b.as_bytes();
    for i in 0..1024 {
        if i >= b.len() || a[i] == 0 && t[i] == 0 {
            break;
        }
        if a[i] != t[i] {
            return false;
        }
    }
    true
}

pub fn strcmp_len(a : &[u8], b : &str, len : usize) -> bool {
    let t = b.as_bytes();
    for i in 0..1024 {
        if i >= len || i >= b.len() || a[i] == 0 && t[i] == 0 {
            break;
        }
        if a[i] != t[i] {
            return false;
        }
    }
    true
}

pub fn get_num_n(str : &[u8], s : usize) -> usize{
    let mut res : usize= 0;
    for i in s..str.len() {
        if str[i] < b'0' || str[i] > b'9' {
            break;
        }
        res *= 10;
        res += (str[i] - b'0') as usize;
    }
    res 
}

pub fn parse(str : &[u8]) -> (usize, usize) {
    if strcmp(str, "Heart") {
        udp_send(1, "beat");
    } else if strcmp_len(str, "PIC", 3) {
        let t = get_num_n(str, 4);
        return (2, t)
    }
    (0, 0)
}