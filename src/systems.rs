use crate::breeder::*;
use crate::components::*;
use crate::creature::*;
use crate::resources::*;
use evo::utils::random;
use evo::{Breeder, NeatBreeder, Pool};
use legion::systems::{CommandBuffer, System};
use legion::world::{SubWorld, World};
use legion::*;

type AntGenetic = Genetic<AntGenome>;

#[system(for_each)]
pub fn update_networks(
    pos: &mut Position,
    net: &mut Network,
    genes: &mut AntGenetic,
    energy: &mut Energy,
    #[resource] config: &Config,
    #[resource] time: &Time,
) {
    let inputs: Vec<f64> = vec![
        time.sin(1.0, 0.0),
        time.sin(1.0, 1.57),
        // pos.x / config.bounds.width as f64 - 0.5,
        // pos.y / config.bounds.height as f64 - 0.5,
    ];

    let out = net.network.run(inputs, 2).unwrap();

    // energy.amt -= out[0].abs() + out[1].abs() * 0.01 * time.dt;

    pos.x += out[0] * time.dt * 60.;
    pos.y += out[1] * time.dt * 60.;

    // genes.fitness -=
    //     ((out[0] - time.sin(1.0, 0.0)).abs()
    //     + (out[1] - time.sin(1.0, 1.571)).abs())
    //      * time.dt * 0.1;

    *pos = pos.clamp(&config.bounds);
    // *pos = pos.wrap(&config.bounds);
}

#[system(for_each)]
pub fn update_energy(e: &mut Energy, g: &mut AntGenetic, #[resource] time: &Time) {
    e.amt -= e.decay * time.dt;
    e.amt -= e.decay * 0.1 * random() as f64 * time.dt;
    if e.amt < 0.0 {
        g.alive = false;
    }
}

#[system]
pub fn remove_dead(
    objects: &mut Query<(Entity, &AntGenetic, &Body)>,
    commands: &mut CommandBuffer,
    world: &mut SubWorld,
    #[resource] pool: &mut AntPool,
    #[resource] config: &Config,
) {
    for (entity, gen, body) in objects.iter(world) {
        if !gen.alive {
            let fitness = body.color.r - body.color.g - body.color.b;
            // let fitness = vec![body.color.r, body.color.g, body.color.b]
            //     .into_iter()
            //     .max_by(|a, b| a.partial_cmp(b).unwrap())
            //     .unwrap() as f32;

            pool.report(
                gen.species_id,
                gen.genome.clone(),
                fitness as f32 + gen.fitness as f32,
            );
            commands.remove(*entity);

            let (id, gene) = pool.next().unwrap();
            let creature = Creature::new(id, gene, config);
            commands.push(creature.components());
        }
    }
}
