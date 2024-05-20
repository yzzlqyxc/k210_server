mod k210handle;
mod httphandles;
use std::{collections::HashMap, net::UdpSocket, sync::{Arc, Mutex}};

use axum::{response::Html, routing::get, Router};
use k210handle::ServerCommu;
type AsyncMap = Arc<Mutex<HashMap<String, ServerCommu>>>;
type AsyncSocket = Arc<Mutex<UdpSocket>>;

#[tokio::main] async fn main() {
    let mp: AsyncMap = Arc::new(Mutex::new(HashMap::new()));
    let socket : AsyncSocket = Arc::new(Mutex::new(UdpSocket::bind("0.0.0.0:12345").unwrap()));

    let a = mp.clone();
    let b = socket.clone();
    let aa = mp.clone();
    let bb = socket.clone();

    let k210 = tokio::task::spawn(k210handle::udps(a, b));
    let https = tokio::task::spawn(httphandles::https(aa, bb));
    tokio::try_join!(k210, https);
}
