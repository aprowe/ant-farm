use tui::layout::Rect;
use evo::Pool;


#[derive(Copy, Clone, Debug)]
pub struct Time {
    pub dt: f64,
    pub elapsed: f64,
}

impl Time {
    pub fn update(self, dt: f64) -> Time {
        Time {
            dt,
            elapsed: self.elapsed + dt
        }
    }

    pub fn tick(self) -> Time {
        self.update(self.dt)
    }

    pub fn sin(&self, period: f64, phase: f64) -> f64 {
        f64::sin(self.elapsed * 2. * 3.14159 / period + phase)
    }
}

pub struct Config {
    pub bounds: Rect,
}


