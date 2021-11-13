pub use evo::utils::random;
pub use evo::utils::random_d;
pub use evo::utils::random_i;

// Multi Type Rect
pub struct RectBase<T>
where
    T: Copy + std::ops::Add<T, Output = T>,
{
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

// Unsigned rect
pub type Rect = RectBase<u16>;

// Float rect
pub type Rectf = RectBase<f64>;

impl<T> RectBase<T>
where
    T: Copy + std::ops::Add<T, Output = T>,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn left(&self) -> T {
        self.x
    }
    pub fn right(&self) -> T {
        self.x + self.width
    }
    pub fn top(&self) -> T {
        self.y
    }
    pub fn bottom(&self) -> T {
        self.y + self.height
    }
}

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
    pub fn rgb(r: f64, b: f64, g: f64) -> Self {
        Color { r, g, b, a: 1.0 }
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

    pub fn dist(&self, c: &Color) -> f64 {
        return self.dist_sq(c).sqrt();
    }

    pub fn dist_sq(&self, c: &Color) -> f64 {
        return (self.r - c.r).powi(2) +
            (self.g - c.g).powi(2) +
            (self.b - c.b).powi(2);
    }
}

impl From<&Color> for Vec<f64> {
    fn from(c: &Color) -> Self {
        vec![c.r, c.g, c.b, c.a]
    }
}

impl From<Color> for Vec<f64> {
    fn from(c: Color) -> Self {
        vec![c.r, c.g, c.b, c.a]
    }
}

impl From<&Color> for [f64; 4] {
    fn from(c: &Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<&Color> for [f32; 3] {
    fn from(c: &Color) -> Self {
        [c.r as f32, c.g as f32, c.b as f32]
    }
}

impl From<[f32; 3]> for Color {
    fn from(c: [f32; 3]) -> Self {
        Color {
            r: c[0] as f64,
            g: c[1] as f64,
            b: c[2] as f64,
            a: 1.0,
        }
    }
}

impl From<&Vec<f64>> for Color {
    fn from(v: &Vec<f64>) -> Self {
        Color {
            r: *v.get(0).unwrap_or(&0.0) as f64,
            b: *v.get(1).unwrap_or(&0.0) as f64,
            g: *v.get(2).unwrap_or(&0.0) as f64,
            a: *v.get(3).unwrap_or(&0.0) as f64,
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

impl From<Color> for (u8, u8, u8) {
    fn from(c: Color) -> (u8, u8, u8) {
        (
            (c.a * c.r * 255.).floor() as u8,
            (c.a * c.b * 255.).floor() as u8,
            (c.a * c.g * 255.).floor() as u8,
        )
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    fn from(c: Color) -> (u8, u8, u8, u8) {
        (
            (c.r * 255.).floor() as u8,
            (c.g * 255.).floor() as u8,
            (c.b * 255.).floor() as u8,
            (c.a * 255.).floor() as u8,
        )
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(pix: (u8, u8, u8, u8)) -> Color {
        Color {
            r:  pix.0 as f64 / 255.0,
            g:  pix.1 as f64 / 255.0,
            b:  pix.2 as f64 / 255.0,
            a:  pix.3 as f64 / 255.0,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a.max(rhs.a),
        }
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a.max(rhs.a),
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a,
        }
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

impl std::ops::AddAssign<(f64, f64)> for Position {
    fn add_assign(&mut self, rhs: (f64, f64)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl std::ops::Add<(f64, f64)> for Position {
    type Output = Position;

    fn add(self, rhs: (f64, f64)) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
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
