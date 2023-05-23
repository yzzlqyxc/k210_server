use smoltcp::Result;
use smoltcp::phy::{self, DeviceCapabilities, Device, Medium};
use smoltcp::time::Instant;

struct K210Phy {
    rx_buffer: [u8; 1536],
    tx_buffer: [u8; 1536],
}

impl<'a> K210Phy {
    fn new() -> K210Phy {
        K210Phy {
            rx_buffer: [0; 1536],
            tx_buffer: [0; 1536],
        }
    }
}

impl<'a> phy::Device<'a> for K210Phy {
    type RxToken = K210PhyRxToken<'a>;
    type TxToken = K210PhyTxToken<'a>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        Some((K210PhyRxToken(&mut self.rx_buffer[..]),
              K210PhyTxToken(&mut self.tx_buffer[..])))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        Some(K210PhyTxToken(&mut self.tx_buffer[..]))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps
    }
}

struct K210PhyRxToken<'a>(&'a mut [u8]);

impl<'a> phy::RxToken for K210PhyRxToken<'a> {
    fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
        where F: FnOnce(&mut [u8]) -> Result<R>
    {
        // TODO: receive packet into buffer
        let result = f(&mut self.0);
        println!("rx called");
        result
    }
}

struct K210PhyTxToken<'a>(&'a mut [u8]);

impl<'a> phy::TxToken for K210PhyTxToken<'a> {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
        where F: FnOnce(&mut [u8]) -> Result<R>
    {
        let result = f(&mut self.0[..len]);
        println!("tx called {}", len);
        // TODO: send packet out
        result
    }
}