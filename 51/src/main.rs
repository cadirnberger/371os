#![no_std]
#![no_main]
#![allow(unconditional_recursion)]
use core::panic::PanicInfo;
mod vga;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    panic!("It is I, a panic!");
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
