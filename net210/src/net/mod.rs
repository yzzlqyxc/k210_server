// use smoltcp::storage;

mod dvc;
mod connection;
mod tcp_connect;

pub fn test() {
    connection::sent();
    tcp_connect::sftp_tcp();
}