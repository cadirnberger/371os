use crate::interrupts::LAST_KEY;
use crate::timer;
pub struct Clock {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milseconds: u64,
}
impl Clock {
    pub fn new(h: u64, m: u64, s: u64, ml: u64) -> Self {
        Clock { hours: h, minutes: m, seconds: s, milseconds: ml }
    }
    pub fn update(&mut self) {
        static mut LAST_TICKS: u64 = 0;
        let ticks = timer::get_ticks();

        let delta_ticks: u64 = unsafe { ticks - LAST_TICKS };
        unsafe { LAST_TICKS = ticks; }

        self.milseconds += delta_ticks;

        while self.milseconds >= 20 {
            self.seconds += 1;
            self.milseconds -= 20;

            if self.seconds >= 60 {
                self.seconds = 0;
                self.minutes += 1;

                if self.minutes >= 60 {
                    self.minutes = 0;
                    self.hours += 1;

                    if self.hours > 12 {
                        self.hours = 1; // 12-hour wrap
                    }
                }
            }
        }
        
    }
}
pub fn init_time() -> Clock {
    //vga::clear()
    crate::println!("Please enter time (hh mm ss): ");
    let mut index = 0;
    let mil = 0;
    let mut sec = 0;
    let mut min = 0;
    let mut hour = 0;
    while index < 6 {
    if let Some(c) = get_key(){
        if let Some(digit) = c.to_digit(10){
        index += 1;
        match index {
                1 => hour = digit as u64,
                2 => hour = hour * 10 + digit as u64,
                3 => min = digit as u64,
                4 => min = min * 10 + digit as u64,
                5 => sec = digit as u64,
                6 => sec = sec * 10 + digit as u64,
                _ => {}
            }

        crate::print!("=> {:02}:{:02}:{:02}\n", hour, min, sec);
        }
      }
    else {
        x86_64::instructions::hlt();  // sleep until next interrupt
    }
    }
Clock::new(hour, min, sec, mil)     
}

fn get_key()-> Option<char>{
    let mut gaurd = LAST_KEY.lock();
    gaurd.take()
}


