use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::print;
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static mut PICS: pic8259::ChainedPics =
    unsafe { pic8259::ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}
impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self as u8)
    }
}
pub fn init_idt() {
    use x86_64::instructions::interrupts;

    
    IDT.load();
    unsafe{
    #[allow(static_mut_refs)]
    PICS.initialize();
    }
    interrupts::enable(); 
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe{ 
        idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);}
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt
    };
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: u64,
) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_handler (
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    #[allow(static_mut_refs)]
    // crate::println!("INTERRUPT: TIMER\n{:#?}", stack_frame);
    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}
extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::Us104Key, HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(_) => {},
            }
        }
    }

    unsafe {
        #[allow(static_mut_refs)]
        PICS.notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: x86_64::structures::idt::PageFaultErrorCode,
) {
    crate::println!("EXCEPTION: PAGE FAULT");
    crate::println!("Accessed Address: {:?}", x86_64::registers::control::Cr2::read());
    crate::println!("Error Code: {:?}", error_code);
    crate::println!("{:#?}", stack_frame);
    loop {
    x86_64::instructions::hlt();
}
}




