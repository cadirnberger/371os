use alloc::collections::vec_deque::VecDeque;
use spin::Mutex;
use x86_64::instructions::interrupts;
use lazy_static::lazy_static;
use crate::println;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

const SNAKE_CHAR: u8 = b'O';
const FOOD_CHAR: u8 = b'a';

const BORDER_H: u8 = 0xC4;
const BORDER_V: u8 = 0xB3;
const CORNER_TL: u8 = 0xDA;
const CORNER_TR: u8 = 0xBF;
const CORNER_BL: u8 = 0xC0;
const CORNER_BR: u8 = 0xD9;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

lazy_static! {
    static ref SNEK: Mutex<VecDeque<(usize, usize)>> = Mutex::new(VecDeque::new());
    pub static ref DIR: Mutex<Direction> =Mutex::new(Direction::Up); 
    }    
    static mut FOOD: (usize, usize) = (10, 10);
    static mut TICK: usize = 0;

// =========================
// VGA WRITING
// =========================

fn write_cell(x: usize, y: usize, byte: u8) {
    let buffer = 0xb8000 as *mut u8;
    let idx = (y * WIDTH + x) * 2;
    unsafe {
        *buffer.add(idx) = byte;
        *buffer.add(idx + 1) = 0x0F;
    }
}

fn clear_cell(x: usize, y: usize) {
    write_cell(x, y, b' ');
}

// =========================
// INIT
// =========================

pub fn init() {
       unsafe{ let mut snek = SNEK.lock();
        snek.push_front((40, 12));
        snek.push_back((39, 12));
        snek.push_back((38, 12));
        crate::println!("SNEK AFTER INIT: len={}", snek.len());
    }
    draw_border();
    draw_food();
    draw_snake();
}

// =========================
// BORDER (DRAW ONCE)
// =========================

fn draw_border() {
    for x in 0..WIDTH {
        write_cell(x, 0, BORDER_H);
        write_cell(x, HEIGHT - 1, BORDER_H);
    }

    for y in 0..HEIGHT {
        write_cell(0, y, BORDER_V);
        write_cell(WIDTH - 1, y, BORDER_V);
    }

    write_cell(0, 0, CORNER_TL);
    write_cell(WIDTH - 1, 0, CORNER_TR);
    write_cell(0, HEIGHT - 1, CORNER_BL);
    write_cell(WIDTH - 1, HEIGHT - 1, CORNER_BR);
}

// =========================
// DRAW
// =========================

fn draw_snake() {
    unsafe{let snek = SNEK.lock();
    
    for &(x, y) in &*snek {
        write_cell(x, y, SNAKE_CHAR);
    }
    }
    
}

fn draw_food() {
    unsafe {
        let (x, y) = FOOD;
        write_cell(x, y, FOOD_CHAR);
    }
}

// =========================
// INPUT (CALL FROM KEYBOARD INTERRUPT)
// =========================
pub fn set_dir(d: Direction) {
    unsafe{
        *DIR.lock() = d;
        
    }
}


// =========================
// UPDATE (CALL FROM TIMER INTERRUPT)
// =========================

pub fn update() {
    unsafe {
        //crate::println!("UPDATE CALLED");
        const SPEED: usize = 5;
        unsafe { TICK += 1; }
        //crate::println!("TICK = {}", unsafe { TICK });
        if unsafe { TICK } % SPEED != 0 {
            return;
        }
        let dir= interrupts::without_interrupts(|| {*DIR.lock()});
        
        interrupts::without_interrupts(|| {
        let mut snek = SNEK.lock();
          let (head_x, head_y) = match snek.front() {
              Some(&(x, y)) => (x, y),
              None => return, // prevents panic
          };
          //crate::println!("SNEK LEN={}, FRONT={:?}", snek.len(), snek.front());
        let new_head: (usize, usize) = match dir {
            Direction::Up => (head_x, head_y.wrapping_sub(1)),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x.wrapping_sub(1), head_y),
            Direction::Right => (head_x + 1, head_y),
          };
        //println!("{:?}", dir);
        //println!("DIR READ = {:?}", dir);
        //crate::println!(
        //"HEAD: ({}, {}) -> NEW: ({}, {})",
        //head_x, head_y, new_head.0, new_head.1
        //);
        // COLLISION
        if is_wall(new_head) || snek.contains(&new_head) {
            quit();
            return;
        }

        // FOOD
        if new_head == unsafe { FOOD } {
            snek.push_front(new_head);
            spawn_food(&snek);
            draw_food();
        } else {
            snek.push_front(new_head);
            let tail = snek.pop_back().unwrap();
            clear_cell(tail.0, tail.1);
        }

        // DRAW NEW HEAD
        write_cell(new_head.0, new_head.1, SNAKE_CHAR);});
    }
}

// =========================
// HELPERS
// =========================

fn is_wall((x, y): (usize, usize)) -> bool {
    x == 0 || x >= WIDTH - 1 || y == 0 || y >= HEIGHT - 1
}

fn spawn_food(snek: &VecDeque<(usize, usize)>) {
    unsafe {
        let mut offset = 0;
        loop {
            let x = (TICK + offset) * 17 % WIDTH;
            let y = (TICK + offset) * 31 % HEIGHT;
            offset += 1;
            if !is_wall((x, y)) && !snek.contains(&(x, y)) {
                FOOD = (x, y);
                break;
            }
        }
    }
}

// =========================
// QUIT
// =========================

fn quit() {
    crate::println!("GAME OVER");
    crate::exit(crate::QEMUExit::Fail);
}






