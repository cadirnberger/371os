pub mod img;
use crate::colors::img::IMG;
pub fn colors() {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    const ROWS: usize = 25;
    const COLS: usize = 80;
    fn rgb_to_vga_color(r: u8, g: u8, b: u8) -> u8 {
    let mut color = if r > 180 || g > 180 || b > 180 { 8 } else { 0 };

    if r > 128 { color |= 0b100; }
    if g > 128 { color |= 0b010; }
    if b > 128 { color |= 0b001; }

    color << 4
    }

    unsafe{
        for row in 0..IMG.len().min(ROWS) {
            for col in 0..IMG[0].len().min(COLS) {
                let (r, g, b) = IMG[row][col];
                let color = rgb_to_vga_color(r, g, b);
                core::ptr::write_volatile(vga_buffer.add((row * COLS + col) * 2 + 1), b' ');
                core::ptr::write_volatile(vga_buffer.add((row * COLS + col) * 2 + 1), color);
            }
        }
    }
    
}
