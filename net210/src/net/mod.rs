mod dvc;
mod cfg;

use dvc::K210Phy;
use smoltcp::iface::Interface;

pub fn test() {
    let device = K210Phy::new();

}