use std::net::UdpSocket;
    
fn main() {
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
