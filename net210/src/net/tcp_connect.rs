use smoltcp::iface::{Interface, Config, SocketSet};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address, HardwareAddress};
use smoltcp::socket::tcp;

mod mock {
    use core::cell::Cell;
    use smoltcp::time::{Duration, Instant};

    #[derive(Debug)]
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

pub fn sftp_tcp() {
    let mut cfg = Config::new();
    let clock = mock::Clock::new();
    cfg.hardware_addr = Some(HardwareAddress::Ethernet(EthernetAddress([0x9c, 0x9c, 0x1f, 0x97, 0xe4, 0x7f])));
    

    let mut dvc = super::dvc::K210Phy::new();
    let mut iface = Interface::new(cfg, &mut dvc);
    iface.update_ip_addrs(|ip_addrs| {
        ip_addrs
            .push(IpCidr::new(IpAddress::v4(192, 168, 69, 1), 24)).unwrap();
    });
    iface.routes_mut().add_default_ipv4_route(Ipv4Address::new(192, 168, 10, 1)).unwrap();


    static mut TCP_SERVER_RX_DATA: [u8; 1024] = [0; 1024];
    static mut TCP_SERVER_TX_DATA: [u8; 1024] = [0; 1024];
    let tcp_rx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_SERVER_RX_DATA[..] });
    let tcp_tx_buffer = tcp::SocketBuffer::new(unsafe { &mut TCP_SERVER_TX_DATA[..] });
    let tcp_socket = tcp::Socket::new(tcp_rx_buffer, tcp_tx_buffer);

    let mut sockets: [_; 2] = Default::default();
    let mut sockets = SocketSet::new(&mut sockets[..]);

    let tcp_handle = sockets.add(tcp_socket);

    let timestamp = clock.elapsed();
    iface.poll(timestamp, &mut dvc, &mut sockets);

    // let mut socket = sockets.get_mut::<tcp::Socket>(tcp_handle);
    // let cx = iface.context(); 
    // let local_port = 12340; 

    // socket.connect(cx, (IpAddress::v4(81, 70, 31, 32), 1234), local_port).unwrap();
}