use crate::breeder::AntGenome;
use crate::components::*;
use crate::resources::Config;

pub type CreatureTuple = (Body, Genetic<AntGenome>, Network);

pub struct Creature {}

impl Creature {
    pub fn new(species_id: i32, gene: AntGenome, c: &Config) -> CreatureTuple {
        (
            Body::random(&c.bounds).color((&gene.color).into()),
            Genetic::<AntGenome>::new(species_id, gene.clone()),
            Network::new(gene.network),
        )
    }
}
