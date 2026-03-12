#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Fail]");
    osirs::exit(osirs::QEMUExit::Fail);
    loop {}
}

fn except_test(){
    x86_64::instructions::interrupts::int3();
    osirs::serial_println!("[Pass] Breakpoint exception handled successfully");
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    osirs::_test_runner(&[&except_test]);
    osirs::exit(osirs::QEMUExit::Fail);
    loop {}
}
