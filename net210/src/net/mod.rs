// use smoltcp::storage;

use crate::net::connection::{at_command, udp_send};

mod dvc;
mod connection;
mod tcp_connect;

pub fn test() {
    let (mut tx, mut rx) = connection::sent();
    println!("wifi connection finish!");
    // tcp_connect::udp_sent();
    at_command("AT+CIPMUX=1\r\n", &mut tx, &mut rx);
    at_command("AT+CIPSTART=1,\"UDP\",\"47.93.124.97\",12345,12345\r\n", &mut tx, &mut rx);
    // at_command("AT+CIPSEND=1,7\r\n", &mut tx, &mut rx);
    // let t = at_command("Hello\r\n", &mut tx, &mut rx);
    udp_send(1, "Hello\r\n", &mut tx, &mut rx);
    udp_send(1, "not Hello", &mut tx, &mut rx);
}
