use crate::breeder::AntGenome;
use crate::components::*;
use crate::resources::Config;
use crate::utils::*;

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

pub type FoodTuple = (Body,);

pub struct Food {}

impl Food {
    pub fn new(emits: Vec<f64>, c: &Config) -> FoodTuple {
        (Body::random(&c.bounds)
            .body_type(BodyType::Food)
            .emits(emits)
            .color(Color::rgb(1.0, 0.0, 0.0)),)
    }
}
