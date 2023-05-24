use k210_hal::time::Bps;
use k210_hal::prelude::_k210_hal_serial_SerialExt;
use core::fmt::Write;

pub fn sent() {
    let ph = k210_hal::Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();
    let pins_wifi = (ph.pins.pin13, ph.pins.pin14);
    let pins_usb= (ph.pins.pin4, ph.pins.pin5);

    let wifi_s = ph.UART1.configure(pins_wifi, Bps(115200), &clocks);
    let usb_s = ph.UART3.configure(pins_usb, Bps(115200), &clocks);

    let (mut tx, rx) = usb_s.split();
    let mut stdout = k210_hal::stdout::Stdout(&mut tx);
    
    writeln!(stdout, "hello");
}