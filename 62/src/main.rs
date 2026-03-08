#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]


mod vga;

//mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    {
        osirs::exit(osirs::QEMUExit::Pass);
    }
    
    osirs::println!("{:081x}",1);
    osirs::println!("{:x}",2);
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}












