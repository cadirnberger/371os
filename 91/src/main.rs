#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(osirs::_test_runner)]

mod vga;

//mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}











