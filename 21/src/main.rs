
fn main() {
    unsafe {
        println!(
            "{}",
            std::mem::transmute::<(usize, usize), &str>((
                &[0x6c6c6548i32, 0x6f77206fi32, 0x21646c72i32] as *const _ as usize,
                12
            ))
        );
    }
}
