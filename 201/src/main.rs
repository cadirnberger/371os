#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(osirs::_test_runner)]

mod vga;
extern crate alloc;
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    osirs::init();
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    osirs::allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
    
    println!("Hello world{}", "!");
    let b = alloc::boxed::Box::new(371);
    println!("Hello box containing {}!", *b);
    println!("Hello box at {:p}!", b);
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}










