#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("{}", _info);
    osirs::exit(osirs::QEMUExit::Pass);
    loop {}
}

#[allow(unconditional_recursion)]
fn recurse() {
    recurse();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    recurse();
    osirs::serial_println!("[Fail] No overflow occurred");
    osirs::exit(osirs::QEMUExit::Fail);
    loop {}
}
