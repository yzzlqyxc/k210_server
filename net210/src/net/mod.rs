// use smoltcp::storage;

mod dvc;
// mod connection;

// pub fn sent() {
//     let (rx_buf, tx_buf) = ([0u8;1024], [0u8; 1024]);
//     let t = smoltcp::storage::RingBuffer::new(storage);
//     let sok = smoltcp::socket::TcpSocket::new(tx_buf, rx_buf);

// }

use smoltcp::phy::{Device, Medium};
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr};
use smoltcp::iface::Interface;
use smoltcp::iface::Config;
use smoltcp::iface::SocketSet;
use smoltcp::socket::tcp;
use smoltcp::time::{Duration, Instant};

mod mock {
    use core::cell::Cell;
    use smoltcp::time::{Duration, Instant};

    pub struct Clock(Cell<Instant>);

    impl Clock {
        pub fn new() -> Clock {
            Clock(Cell::new(Instant::from_millis(0)))
        }

        pub fn advance(&self, duration: Duration) {
            self.0.set(self.0.get() + duration)
        }

        pub fn elapsed(&self) -> Instant {
            self.0.get()
        }
    }
}


pub fn test() {
    let clock = mock::Clock::new();
    // let device = Loopback::new(Medium::Ethernet);
    let mut device = dvc::K210Phy::new();

    let mut config = match device.capabilities().medium {
        Medium::Ethernet => {
            Config::new()
        }
        Medium::Ip => Config::new(),
    };
    let mut iface = Interface::new(config, &mut device);
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(127, 0, 0, 1), 8))
            .unwrap();
    });

    //创建socket
    let server_socket = {
        static mut TCP_SERVER_RX_DATA: [u8; 1024] = [0; 1024];
        static mut TCP_SERVER_TX_DATA: [u8; 1024] = [0; 1024];
        let tcp_rx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_SERVER_RX_DATA[..] });
        let tcp_tx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_SERVER_TX_DATA[..] });
        tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer)
    };

    let client_socket = {
        static mut TCP_CLIENT_RX_DATA: [u8; 1024] = [0; 1024];
        static mut TCP_CLIENT_TX_DATA: [u8; 1024] = [0; 1024];
        let tcp_rx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_CLIENT_RX_DATA[..] });
        let tcp_tx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_CLIENT_TX_DATA[..] });
        tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer)
    };

    let mut sockets: [_; 2] = Default::default();
    let mut sockets = SocketSet::new(&mut sockets[..]);
    let server_handle = sockets.add(server_socket);
    let client_handle = sockets.add(client_socket);

    let mut did_listen = false;
    let mut did_connect = false;
    let mut done = false;
    while !done && clock.elapsed() < Instant::from_millis(10_000) {
        iface.poll(clock.elapsed(), &mut device, &mut sockets);

        let mut socket = sockets.get_mut::<tcp::Socket>(server_handle);
        if !socket.is_active() && !socket.is_listening() {
            if !did_listen {
                // debug!("listening");
                socket.listen(1234).unwrap();
                did_listen = true;
            }
        }

        if socket.can_recv() {
            socket.close();
            done = true;
        }

        let mut socket = sockets.get_mut::<tcp::Socket>(client_handle);
        let cx = iface.context();
        if !socket.is_open() {
            if !did_connect {
                socket
                    .connect(cx, (IpAddress::v4(127, 0, 0, 1), 1234), 65000)
                    .unwrap();
                did_connect = true;
            }
        }

        if socket.can_send() {
            socket.send_slice(b"0123456789abcdef").unwrap();
            socket.close();
        }

        match iface.poll_delay(clock.elapsed(), &sockets) {
            Some(delay) => {
                clock.advance(delay)
            }
            None => clock.advance(Duration::from_millis(1)),
        }
    }
}