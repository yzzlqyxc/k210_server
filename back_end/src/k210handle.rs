use std::{cmp::min, collections::HashMap, net::UdpSocket, sync::{Arc, Mutex}, time::Duration};
use std::net::SocketAddr;
use crate::{AsyncMap, AsyncSocket};

use tokio::time::sleep;

static mut HEART : usize = 0;

#[derive(Debug)]
pub struct ServerCommu {
    addr : SocketAddr,
    history : Vec<String>, 
    timestamp : usize, 
}

impl ServerCommu {
    fn new(addr : SocketAddr, str : String) -> Self {
        Self {
            addr, 
            history : Vec::from([str]),
            timestamp : unsafe {HEART},
        }
    }
    fn add_commu(&mut self, str : String) {
        self.history.push(str);
    }
    fn heartup(&mut self) {
        self.timestamp = unsafe{HEART};
    }
    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }
    pub fn send_msg(&mut self, str : String, socket : AsyncSocket) {
        let t = socket.clone();
        let soc = t.lock().unwrap();
        println!("{:?}", self);
        soc.send_to(&str.as_bytes()[..str.len()], self.addr).unwrap();
        self.add_commu(str);
    }
    pub fn send_pic(&mut self, str : &Vec<u8>, socket : AsyncSocket) {
        println!("iiiiiinnnn");
        let t = socket.clone();
        let soc = t.lock().unwrap();
        let len = str.len();
        let times = len / 100 + ((len % 100 != 0) as usize);
        let head = format!("PIC {}", times);
        println!("{}", head);
        soc.send_to(&head.as_bytes()[0..head.len()], self.addr).unwrap();

        let lag = 30;
        std::thread::sleep(Duration::from_millis(lag));
        for i in 0..times {
            println!("have send{}", i);
            soc.send_to(&str[(i * 100)..min(i * 100 + 100, str.len())], self.addr).unwrap();
            std::thread::sleep(Duration::from_millis(lag));
        }
    }   
}

async fn heartbeats(mp : AsyncMap, socket : AsyncSocket) {
    return ;
    loop {
        {unsafe{HEART += 1;}
        let t = mp.clone();
        let mut mp_clone = t.lock().unwrap();
        let addrs : Vec<String> = mp_clone.keys().cloned().collect();
        println!("{:?}", addrs);
        for i in addrs {
            let v = mp_clone.get_mut(&i).unwrap();
            unsafe {
                if HEART - v.timestamp >= 10 {
                    mp_clone.remove(&i);
                    continue;
                }
            }
            v.send_msg("Heart".to_owned(), socket.clone());
        }}
        sleep(Duration::from_secs(1)).await; 
    }
}

async fn handling_incoming(socket : AsyncSocket, mp : AsyncMap) {
    loop {
        let mut buf = [0; 1024];
        let t = {
        let t = Arc::clone(&socket);
        let soc = t.lock().unwrap();

        soc.recv_from(&mut buf)
        };
        if t.is_err() {
            sleep(Duration::from_millis(50)).await;
            continue;
        }
        let (num_bytes, src_addr) = t.unwrap();


        let received_str = std::str::from_utf8(&buf[..num_bytes]).unwrap();
        let addr_string = src_addr.to_string();

        let t = mp.clone();
        let mut mp_clone = t.lock().unwrap();
        println!("Received message from {}: {}", src_addr, received_str);
        mp_clone.entry(addr_string.clone())
              .and_modify(|x| {x.add_commu(received_str.to_owned())})
              .or_insert(ServerCommu::new(src_addr.clone(), received_str.to_owned()));

        if received_str == "beat" {
            mp_clone.entry(addr_string).and_modify(|x| {x.heartup();});
        }
    }
}

pub async fn udps(mp : AsyncMap, socket : AsyncSocket) {

    println!("Server listening on port 12345...");

    let a = socket.clone();
    {
        let ac = a.lock().unwrap();
        ac.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    }
    let b = mp.clone();
    let aa = socket.clone();
    let bb = mp.clone();


    let heartbeating = tokio::spawn(async move {
        heartbeats(b, a).await;
    });
    let handling= tokio::spawn(async move {
        handling_incoming(aa, bb).await;
    });
    tokio::join!(handling, heartbeating);
}

