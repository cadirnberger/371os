fn main() {
    let byte: [i32;3] = [0x6c6c6548, 0x6f77206f, 0x21646c72];
    unsafe {
        let st: &str =  std::mem::transmute::<(usize,usize), &str>((&byte as *const _ as usize,12));
        println!("{}", st);
    }
}
