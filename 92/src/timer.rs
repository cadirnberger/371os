static mut TICKS: u64 = 0;

pub fn tick() {
    unsafe {
        TICKS += 1;
    }
}
pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}
