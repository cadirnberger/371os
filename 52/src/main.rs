#![no_std]
#![no_main]
#![allow(unconditional_recursion)]
mod vga;
mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::colors();
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
