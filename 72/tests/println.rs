#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Fail]");
    osirs::exit(osirs::QEMUExit::Fail);
    loop {}
}

pub fn print_test() {
    osirs::println!("Hi I am caden");
    osirs::serial_println!("[Pass]");
}
fn test_println_many() {
    for _ in 0..200 {
        osirs::println!("scroll test");
    }
    osirs::serial_println!("[Pass]");
}

fn test_println_wrap() {
    osirs::println!("This line is very very very very very very very very very very very long");
    osirs::serial_println!("[Pass]");
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::_test_runner(&[&print_test,&test_println_many, &test_println_wrap,]);
    osirs::exit(osirs::QEMUExit::Pass);    
    loop {}
}
