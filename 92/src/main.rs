#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(osirs::_test_runner)]
use osirs::clock::init_time;
mod vga;
//mod clock;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    let mut clock = init_time();
    let mut last_second = 0;
    loop {
        x86_64::instructions::hlt();
        clock.update();

        //if clock.seconds != last_second {
        //    last_second = clock.seconds;

        osirs::print!(
            "Time: {:02}:{:02}:{:02}\n",
            clock.hours,
            clock.minutes,
            clock.seconds
        );
        //}
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}











