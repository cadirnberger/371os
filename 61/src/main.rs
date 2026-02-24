#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
//mod colors;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main.");
    #[cfg(test)]
    test_runner(&[]);
    loop {}
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
pub fn test_runner(_tests: &[&dyn Fn()]) {
    let fs = [_ex]; // Array of test functions
    for i in 0..fs.len() {
        print!("Running test case {:0x}", i);
        fs[i]();
        println!("Success.");
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

fn _bye() {
    println!("Goodbye space!");
    return;
}
fn _ex() {
    assert!(true);
}
