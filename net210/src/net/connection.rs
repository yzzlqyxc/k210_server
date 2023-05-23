use k210_hal::time::Bps;
// use k210_hal::prelude::_k210_hal_serial_SerialExt;


pub fn connect() {
    let ph = k210_hal::Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();
    let pins = (ph.pins.pin13, ph.pins.pin14);

    let t = ph.UART1.configure(pins, Bps(115200), &clocks);

}