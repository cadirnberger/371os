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

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    #[cfg(test)]
    osirs::exit(osirs::QEMUExit::Pass);

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
