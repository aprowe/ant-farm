use crate::utils::{Color, Position, Rect};
use evo::utils::random;
use evo::NeatGenome;
use evo::NeatNetwork;

///////////////////////////////
/// Body
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
    pub energy: Energy,
    pub color: Color,
    pub position: Position,
    pub theta: f64,
}

impl Body {
    pub fn random(rect: &Rect) -> Body {
        Body {
            energy: Default::default(),
            position: Position::random(rect),
            color: Color::random(),
            theta: random(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn detect(&self, b2: &Body, dist: f64, angle: f64) -> f64 {
        let d = self.position.dist_sq(&b2.position);
        if d > dist * dist {
            return 0.0;
        }

        if (self.position.atan2(&b2.position) - self.theta).abs() > angle {
            return 0.0;
        }

        return 1.0;
    }
}

//////////////////////////////////
/// Energy
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Energy {
    pub amt: f64,
    pub decay: f64,
}

impl Default for Energy {
    fn default() -> Self {
        Self {
            amt: 1.0,
            decay: 0.1,
        }
    }
}

//////////////////////////////////
/// Network
///
#[derive(Clone, Debug)]
pub struct Network {
    pub network: NeatNetwork,
    pub inputs: i32,
    pub outputs: i32,
    pub input_state: Vec<f64>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            inputs: 2,
            outputs: 2,
            network: NeatNetwork::random(2, 2),
            input_state: vec![0.0; 2],
        }
    }
}

impl Network {
    pub fn new(genome: NeatGenome) -> Self {
        Self {
            inputs: 2,
            outputs: 2,
            network: NeatNetwork::from(genome),
            input_state: vec![0.0; 2],
        }
    }
}

//////////////////////////////////
/// Genetic
///
#[derive(Clone, Debug)]
pub struct Genetic<T: Clone> {
    pub species_id: i32,
    pub genome: T,
    pub fitness: f64,
    pub alive: bool,
}

impl<T> Genetic<T>
where
    T: Clone,
{
    pub fn new(species_id: i32, genome: T) -> Self {
        Self {
            species_id,
            genome,
            fitness: 0.0,
            alive: true,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect() {
        let b1 = Body {
            energy: Default::default(),
            position: Position { x: 0.0, y: 0.0 },
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            theta: 0.0,
        };

        let b2 = Body {
            energy: Default::default(),
            position: Position { x: 1.0, y: 0.0 },
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            theta: 0.0,
        };

        assert_eq!(b1.detect(&b2, 1.1, 0.01), 1.0);
        assert_eq!(b1.detect(&b2, 1.1, 0.01), 0.0);
    }
}
