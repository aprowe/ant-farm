use evo::utils::random;
use evo::NeatNetwork;
use evo::NeatGenome;
use tui::layout::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
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

impl Color {
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
        } else
        if self.g > self.b {
            self.g
        } else {
            self.b
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn clamp(&self, rect: &Rect) -> Self {
        Self {
            x: self.x.clamp(rect.left() as f64, rect.right() as f64),
            y: self.y.clamp(rect.top() as f64, rect.bottom() as f64),
        }
    }

    pub fn wrap(&self, rect: &Rect) -> Self {
        let Position {x, y } = *self;
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
            }
        }
    }

    pub fn random(rect: &Rect) -> Self {
        Self {
            x: random() as f64 * rect.width as f64,
            y: random() as f64 * rect.height as f64,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
    pub color: Color
}

impl Body {
    pub fn random() -> Body {
        Body {
            color: Color::random()
        }
    }

    pub fn new(color: Color) -> Body {
        Body {
            color: Color {
                a: 1.0,
                ..color
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Network {
    pub network: NeatNetwork,
    pub inputs: i32,
    pub outputs: i32,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            inputs: 2,
            outputs: 2,
            network: NeatNetwork::random(2, 2),
        }
    }
}

impl Network {
    pub fn new(genome: NeatGenome) -> Self {
        Self {
            inputs: 2,
            outputs: 2,
            network: NeatNetwork::from(genome),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub amt: f64,
    pub decay: f64,
}

#[derive(Clone, Debug)]
pub struct Genetic<T: Clone> {
    pub species_id: i32,
    pub genome: T,
    pub fitness: f64,
    pub alive: bool,
}

impl<T> Genetic<T> where T: Clone {
    pub fn new(species_id: i32, genome: T) -> Self {
        Self {
            species_id,
            genome,
            fitness: 0.0,
            alive: true,
        }
    }
}

impl Default for Energy {
    fn default() -> Self {
        Self {
            amt: 1.0,
            decay: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    dx: f64,
    dy: f64,
}

