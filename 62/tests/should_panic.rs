#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::exit(osirs::QEMUExit::Pass);
    loop {}
}

fn bad() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&bad]);
    osirs::exit(osirs::QEMUExit::Fail);
    loop {}
}
