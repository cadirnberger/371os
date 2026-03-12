static mut IDT: x86_64::structures::idt::InterruptDescriptorTable =
    x86_64::structures::idt::InterruptDescriptorTable::new();
#[allow(static_mut_refs)]
pub fn init_idt() {
    unsafe{
    
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.load();
    }
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame)
{
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
