#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod serial;
//mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    //println!("I'm main.");
    #[cfg(test)]
    test_runner(&[]);
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Failed]");
    serial_println!("Error: {}\n", info);
    unsafe{
    x86_64::instructions::port::Port::new(0xf4).write(0xFu32);
    }
    loop {}
}
#[cfg(test)]
pub fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex, _bad]; // Array of test functions
    for i in 0..fs.len() {
        serial_print!("Beginning test 0x{:02x}...", i);
        fs[i]();
        serial_println!("[Pass]");
    }
    // Exit QEMU after tests
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(0xAu32);
    }
}
fn _hi() {
    println!("Hello world!");
    return;
}

fn _bad() {
    assert!(false);
}
fn _ex() {
    assert!(true);
}











