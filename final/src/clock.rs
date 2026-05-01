
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



