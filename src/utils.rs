pub use evo::utils::random;
pub use evo::utils::random_d;
pub use evo::utils::random_i;
pub use tui::layout::Rect;

/////////////////////////////////////
///
/// Color struct
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn rgb(r: f64, b:f64, g:f64) -> Self {
        Color {
            r, g, b, a: 1.0
        }
    }

    pub fn random() -> Color {
        Color {
            r: random() as f64,
            b: random() as f64,
            g: random() as f64,
            a: random() as f64,
        }
    }

    pub fn max(&self) -> f64 {
        if self.r > self.g && self.r > self.b {
            self.r
        } else if self.g > self.b {
            self.g
        } else {
            self.b
        }
    }
}

impl From<&Color> for Vec<f64> {
    fn from(c: &Color) -> Self {
        vec![c.r, c.g, c.b, c.a]
    }
}

impl From<&Color> for [f64; 4] {
    fn from(c: &Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<&Vec<f64>> for Color {
    fn from(v: &Vec<f64>) -> Self {
        Color {
            r: v[0] as f64,
            b: v[1] as f64,
            g: v[2] as f64,
            a: v[3] as f64,
        }
    }
}

impl From<f64> for Color {
    fn from(c: f64) -> Color {
        Color {
            r: 1.0,
            g: c / std::f64::consts::TAU + 0.5,
            b: c / std::f64::consts::TAU + 0.5,
            a: 1.0,
        }
    }
}

impl From<Color> for tui::style::Color {
    fn from(c: Color) -> tui::style::Color {
        tui::style::Color::Rgb(
            (c.a * c.r * 255.).floor() as u8,
            (c.a * c.b * 255.).floor() as u8,
            (c.a * c.g * 255.).floor() as u8,
        )
    }
}

/////////////////////////////////////
///
/// Position Struct
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn dist_sq(&self, p: &Position) -> f64 {
        (self.x - p.x).powi(2) + (self.y - p.y).powi(2)
    }

    pub fn atan2(&self, p: &Position) -> f64 {
        (self.y - p.y).atan2(self.y - p.y)
    }

    pub fn clamp(&self, rect: &Rect) -> Self {
        Self {
            x: self.x.clamp(rect.left() as f64, rect.right() as f64),
            y: self.y.clamp(rect.top() as f64, rect.bottom() as f64),
        }
    }

    pub fn wrap(&self, rect: &Rect) -> Self {
        let Position { x, y } = *self;
        Self {
            x: if x > rect.right() as f64 {
                x - rect.width as f64
            } else if x < rect.left() as f64 {
                x + rect.width as f64
            } else {
                x
            },
            y: if y > rect.right() as f64 {
                y - rect.height as f64
            } else if y < rect.left() as f64 {
                y + rect.height as f64
            } else {
                y
            },
        }
    }

    pub fn random(rect: &Rect) -> Self {
        Self {
            x: random() as f64 * rect.width as f64,
            y: random() as f64 * rect.height as f64,
        }
    }

    pub fn advance(&self, r: f64, theta: f64) -> Position {
        Self {
            x: self.x + r * theta.sin(),
            y: self.y + r * theta.cos(),
        }
    }
}

//////////////////////////////////
/// Float Utils
///
pub trait FloatUtils {
    fn wrap(self) -> Self;
}

impl FloatUtils for f64 {
    fn wrap(self) -> Self {
        use std::f64::consts::TAU;

        if self > TAU {
            self - TAU
        } else if self < -TAU {
            self + TAU
        } else {
            self
        }
    }
}
