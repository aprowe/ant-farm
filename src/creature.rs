use crate::resources::Config;
use crate::components::*;
use crate::breeder::{AntBreeder, AntGenome};

pub type CreatureTuple = (Position, Genetic<AntGenome>, Network, Energy, Body);

pub struct Creature {
    pos: Position,
    genetic: Genetic<AntGenome>,
    network: Network,
    energy: Energy,
    body: Body,
}

impl From<CreatureTuple> for Creature {
    fn from(c: CreatureTuple) -> Self {
        Self {
            pos: c.0,
            genetic: c.1,
            network: c.2,
            energy: c.3,
            body: c.4,
        }
    }
}

impl From<Creature> for CreatureTuple {
    fn from(c: Creature) -> Self {
        (c.pos, c.genetic, c.network, c.energy, c.body)
    }
}

impl Creature {
    pub fn components(self) -> CreatureTuple {
        self.into()
    }

    pub fn new (species_id: i32, gene: AntGenome, c: &Config) -> Creature {
        Creature {
            pos: Position::random(&c.bounds),
            genetic: Genetic::<AntGenome>::new(species_id, gene.clone()),
            network: Network::new(gene.network),
            energy: Energy::default(),
            body: Body::new((&gene.color).into()),
        }
    }
}
