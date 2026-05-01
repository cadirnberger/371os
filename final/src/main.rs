#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(osirs::_test_runner)]
mod vga;
use osirs::allocator;
extern crate alloc;
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    osirs::allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    allocator::init_heap(&mut mapper, &mut frame_allocator);
    osirs::snake::init();
    x86_64::instructions::interrupts::enable();
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}










