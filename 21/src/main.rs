
fn main() {
    let byte: [i32;3] = [0x6c6c6548i32, 0x6f77206fi32, 0x21646c72i32];
    unsafe {
        println!(
            "{}",
            std::mem::transmute::<(usize,usize), &str>((
                &byte as *const _ as usize,
                12
            ))
        );
    }
}
