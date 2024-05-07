// use smoltcp::storage;

use crate::net::connection::{at_command, udp_send};
pub mod connection;

pub fn test() {
    at_command("AT+CWLAP\r\n");
    at_command("AT+CWJAP_CUR=\"test\",\"12344321\"\r\n");
    // loop {
        // let t = at_command("AT+CWJAP_CUR=\"test\",\"12344321\"\r\n");
    //     if t.is_ok() {
    //         break;
    //     }
    // }
    at_command("AT+PING=\"47.93.124.97\"\r\n");
    at_command("AT+CIPMUX=1\r\n");
    at_command("AT+CIPSTART=1,\"UDP\",\"47.93.124.97\",12345,12345\r\n");
    // at_command("AT+CIPSEND=1,7\r\n", &mut tx, &mut rx);
    // let t = at_command("Hello\r\n", &mut tx, &mut rx);
    udp_send(1, "Hello");
}
