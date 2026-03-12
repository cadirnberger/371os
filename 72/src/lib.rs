#![no_main]
#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::_test_runner)]
#![reexport_test_harness_main = "test_main"]
pub mod vga;
pub mod serial;
pub mod interrupts;
pub enum QEMUExit {
    Pass = 0xA,
    Fail = 0xF
} 
 pub fn exit(exit_code: QEMUExit){
    unsafe{
    let mut port = x86_64::instructions::port::Port::new(0xf4);
    port.write(exit_code as u32);
    }
 }
#[cfg(test)]
#[panic_handler]
  fn test_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Failed]");
    serial_println!("Error: {}\n", info);
    exit(QEMUExit::Fail);  
    loop {}
  }


pub fn _test_runner(_tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", _tests.len());

    for test in _tests {
        test();
    }
    exit(QEMUExit::Pass);
}
pub fn init() {
    interrupts::init_idt();
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    loop {}
}
