use crate::utils::{Color, Position, Rect};
use evo::utils::random;
use evo::NeatGenome;
use evo::NeatNetwork;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum BodyType {
    Food,
    Creature
}

///////////////////////////////
/// Body {{{1
///
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub body_type: BodyType,
    pub energy: Energy,
    pub color: Color,
    pub position: Position,
    pub theta: f64,
    pub emits: Vec<f64>
}

impl Body {
    pub fn random(rect: &Rect) -> Body {
        Body {
            body_type: BodyType::Creature,
            position: Position::random(rect),
            color: Color::random(),
            theta: random() * std::f64::consts::TAU,
            ..Default::default()
        }
    }

    /// Builder methods
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn body_type(mut self, body_type: BodyType) -> Self {
        self.body_type = body_type;
        self
    }
    pub fn emits(mut self, emits: Vec<f64>) -> Self {
        self.emits = emits;
        self
    }


    // Distance to another body
    pub fn dist_sq(&self, b: &Body) -> f64 {
        self.position.dist_sq(&b.position)
    }

    // Detect another body
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

impl Default for Body {
    fn default() -> Self {
        Body {
            body_type: BodyType::Creature,
            energy: Default::default(),
            color: Color::random(),
            position: Position {
                x: 0.,
                y: 0.,
            },
            theta: random(),
            emits: vec![],
        }
    }
}

//////////////////////////////////
/// Energy {{{ 1
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
/// Network {{{1
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
/// Genetic {{{1
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
            position: Position { x: 0.0, y: 0.0 },
            ..Default::default()
        };

        let b2 = Body {
            position: Position { x: 1.0, y: 0.0 },
            ..Default::default()
        };

        assert_eq!(b1.detect(&b2, 1.1, 0.01), 1.0);
        assert_eq!(b1.detect(&b2, 1.1, 0.01), 0.0);
    }
}
