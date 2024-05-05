use crate::net::connection::{udp_send, AA};

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

pub fn parse(str : &[u8]) {
    if strcmp(str, "Heart") {
        udp_send(1, "beat");
    }
}