use evo::{NeatBreeder, Pool, VecBreeder, derive_breeder, Breeder};

//////////////////////////////////
/// Primary Creature Breeder Struct
///
#[derive_breeder]
pub struct AntBreeder {
    #[breeder(0.1)]
    pub color: VecBreeder,

    #[breeder(0.9)]
    pub network: NeatBreeder
}

/// type Aliases
pub type AntGenome = <AntBreeder as Breeder>::Genome;
pub type AntPool = Pool<AntBreeder>;

impl Default for AntBreeder {
    fn default() -> Self {
        let mut breeder = VecBreeder::default();
        breeder.size = 4;
        breeder.min = 0.0;
        breeder.max = 1.0;

        Self {
            color: breeder,
            network: NeatBreeder {
                inputs: 5,
                outputs: 3,
                ..NeatBreeder::default()
            }
        }
    }
}
