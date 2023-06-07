use smoltcp::phy::{self, DeviceCapabilities,  Medium};
use smoltcp::time::Instant;

pub struct K210Phy {
    rx_buffer: [u8; 1024],
    tx_buffer: [u8; 1024],
}

impl<'a> K210Phy {
    pub fn new() -> K210Phy {
        K210Phy {
            rx_buffer: [0; 1024],
            tx_buffer: [0; 1024],
        }
    }
}

impl phy::Device for K210Phy {
    type RxToken<'a> = K210PhyRxToken<'a> where Self: 'a;
    type TxToken<'a> = K210PhyTxToken<'a> where Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        Some((K210PhyRxToken(&mut self.rx_buffer[..]),
              K210PhyTxToken(&mut self.tx_buffer[..])))
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(K210PhyTxToken(&mut self.tx_buffer[..]))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1024;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps
    }
}

pub struct K210PhyRxToken<'a>(&'a mut [u8]);

impl<'a> phy::RxToken for K210PhyRxToken<'a> {
    fn consume<R, F>(mut self, f: F) -> R
        where F: FnOnce(&mut [u8]) -> R
    {
        // TODO: receive packet into buffer
        let result = f(&mut self.0);
        println!("{:?}", self.0);
        result
    }
}

pub struct K210PhyTxToken<'a>(&'a mut [u8]);

impl<'a> phy::TxToken for K210PhyTxToken<'a> {
    fn consume<R, F>(self, len: usize, f: F) -> R
        where F: FnOnce(&mut [u8]) -> R
    {
        let result = f(&mut self.0[..len]);
        println!("tx called {}", len);
        // TODO: send packet out
        result
    }
}