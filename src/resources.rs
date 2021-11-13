use crate::utils::Rect;
use std::f64::consts;

//////////////////////////////////
/// Time Related Info
///
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
        f64::sin(self.elapsed * consts::TAU / period + phase * consts::TAU)
    }

    pub fn cos(&self, period: f64, phase: f64) -> f64 {
        f64::cos(self.elapsed * consts::TAU / period + phase * consts::TAU)
    }
}

//////////////////////////////////
/// World Config
///
pub struct Config {
    pub bounds: Rect,
}


