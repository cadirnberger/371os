const SIZE: usize = 1024;
static mut BUS: [u8; SIZE] = [0; SIZE];
static mut INIT: bool = false;

fn init() {
    unsafe {
        if INIT { return; }
        for i in 0..SIZE { BUS[i] = 0; }
        INIT = true;
    }
}

pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        init();
        let mut offset = 0;
        while offset + s <= SIZE {
            if BUS[offset..offset+s].iter().all(|&b| b == 0) {
                for i in 0..s { BUS[offset + i] = 0xFF; } // mark as allocated
                return Some(offset);
            }
            offset += 1;
        }
        None
    }
}

pub fn setter(val: i32, ptr: usize) {
    unsafe {
        let bytes = val.to_ne_bytes();
        for i in 0..4 { BUS[ptr + i] = bytes[i]; }
    }
}

pub fn getter(ptr: usize) -> i32 {
    unsafe {
        let mut bytes = [0u8; 4];
        for i in 0..4 { bytes[i] = BUS[ptr + i]; }
        i32::from_ne_bytes(bytes)
    }
}
