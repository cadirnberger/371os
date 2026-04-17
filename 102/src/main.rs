#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(osirs::_test_runner)]

mod vga;
use osirs::memory;
//mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator = unsafe { osirs::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
osirs::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}











