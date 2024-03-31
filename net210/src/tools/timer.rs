pub const CLOCK_FREQ : usize = 400000000;
pub const MSEC_PER_SEC : usize = 1000;


pub fn get_time_ms() -> usize {
    let rr : usize;
    unsafe {
        // core::arch::asm!("rdcycle {0}", out(reg) rr);
        core::arch::asm!("csrrs {0}, 0xC00, x0", out(reg) rr);
    }
    rr / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn get_time() -> usize {
    get_time_ms() / MSEC_PER_SEC
}
 
pub fn sleep(t : usize) {
    let now = get_time_ms();
    println!("{}", now);
    while get_time_ms() - now <= t  {
        println!("{}", get_time_ms());
    }
    return;
}