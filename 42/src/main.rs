#![no_std]
#![no_main]
#![allow(unconditional_recursion)]
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let ints: [i32; 3] = [1819043144, 1870078063, 560229490];

    // VGA text buffer
    let vga_buffer = 0xb8000 as *mut u8;
    let mut offset = 0;

    unsafe {
        for &i in &ints {
            let bytes = i.to_le_bytes();
            for &b in &bytes {
                // write ASCII byte
                core::ptr::write_volatile(vga_buffer.add(offset * 2), b);
                // write color byte: bright white on black
                core::ptr::write_volatile(vga_buffer.add(offset * 2 + 1), 0x0F);
                offset += 1;
            }
        }
    }
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
