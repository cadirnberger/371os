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
    println!("Level 4 page table at: {:?}", x86_64::registers::control::Cr3::read().0.start_address());
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}











