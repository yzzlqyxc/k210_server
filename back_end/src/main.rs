use std::{collections::HashMap, net::UdpSocket, thread::sleep, time::Duration};

use axum::{routing, Router};

async fn udps() {
    
    let socket = UdpSocket::bind("0.0.0.0:12345").unwrap();
    println!("Server listening on port 12345...");
    let mut buf = [0; 1024];

    loop {
        // Receive data from a client
        let (num_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();

        // Convert the received bytes to a string
        let received_str = std::str::from_utf8(&buf[..num_bytes]).unwrap();

        println!("Received message from {}: {}", src_addr, received_str);

        // Echo the received message back to the client
        for _ in 0..3 {
            socket.send_to(&buf[..num_bytes], src_addr).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main] async fn main() {
    tokio::spawn(async move {
        udps().await;
    });

    tracing_subscriber::fmt::init();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", routing::get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap(); 
}
