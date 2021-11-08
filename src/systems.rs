use std::sync::Arc;
use std::sync::Mutex;

use crate::breeder::*;
use crate::components::*;
use crate::creature::*;
use crate::field::*;
use crate::field_render::FieldRenderer;
use crate::resources::*;
use crate::utils::*;
use evo::utils::random;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::*;

type AntGenetic = Genetic<AntGenome>;

#[system]
#[read_component(Body)]
#[read_component(AntGenetic)]
#[read_component(Entity)]
#[read_component(Network)]
#[write_component(AntGenetic)]
#[write_component(Body)]
pub fn detect(
    objects: &mut Query<&Body>,
    world: &mut SubWorld,
    #[resource] config: &Config,
    #[resource] time: &Time,
) {
    let mut creatures = vec![];
    let mut food = vec![];
    for (entity, body, net) in <(Entity, &Body, &Network)>::query().iter(world) {
        for (entity2, body2) in <(Entity, &Body)>::query()
            .iter(world)
            .filter(|b| b.1.body_type == BodyType::Food)
        {
            if body.dist_sq(body2) < 2. * 2. {
                creatures.push((*entity, body2.energy.amt));
                food.push((*entity2, -body2.energy.amt));
            }
        }
    }


    // body.energy.amt += body2.energy.amt;
    // body2.energy.amt = 0.0;

    for (entity, amt) in creatures.into_iter() {
        let (b, g) = <(&mut Body, &mut AntGenetic)>::query()
            .get_mut(world, entity)
            .unwrap();
        b.energy.amt += amt;
        g.fitness += 1.0;
    }

    for (entity, amt) in food.into_iter() {
        let b = <&mut Body>::query().get_mut(world, entity).unwrap();
        b.energy.amt -= amt;
    }

    // for (entity, amt) in f {
    //     let obj = <&mut Network>::query().get_mut(world, entity).unwrap();
    //     obj.input_state[0] = amt;
    // }
}

#[system(for_each)]
pub fn update_emitters(
    body: &Body,
    #[resource] time: &Time,
    #[resource] field: &mut Field,
    #[resource] config: &Config,
) {
    if body.emits.len() != field.slots {
        return;
    }

    let x = body.position.x / config.bounds.width as f64;
    let y = 1.0 - body.position.y / config.bounds.height as f64;

    field.add_norm(x, y, body.emits.clone());
}

#[system(for_each)]
pub fn update_networks(
    net: &Network,
    genes: &mut AntGenetic,
    body: &mut Body,
    #[resource] config: &Config,
    #[resource] time: &Time,
    #[resource] field: &Field,
    // #[resource] field: &Arc<Mutex<FieldRenderer>>,
) {
    // let (px, py, _) = (1.0, 0.0, 0.0);
    // pos.x / config.bounds.width as f64 - 0.5,
    // pos.y / config.bounds.height as f64 - 0.5,
    // pos.y / config.bounds.height as f64 - 0.5,
    // );
    // let inputs: Vec<f64> = vec![0.,0.];
    // let inputs: Vec<f64> = net.input_state.clone();
    // let inputs: Vec<f64> = net.input_state.clone();
    // vec![
    // body.detect(body, pos, &Position {x: 0.0, y: 0.0}, 50.0, 0.5),
    // body.detect(body, pos, &Position {x: 0.0, y: 0.0}, 50.0, 0.5)
    // time.sin(1.0, 0.0),
    // time.sin(1.0, 1.57),
    // pos.x / config.bounds.width as f64,
    // pos.y / config.bounds.height as f64,
    // 0.0
    // pos.y / config.bounds.height as f64 - 0.5,
    // ];

    let x = body.position.x / config.bounds.width as f64;
    let y = 1.0 - body.position.y / config.bounds.height as f64;

    // let vals = field.lock().unwrap().get(x.round() as usize, y.round() as usize);

    // get derivative input
    let inputs = field.get_derivative(x, y)
        .iter()
        .flat_map(|a| vec![a.0, a.1])
        .collect::<Vec<f64>>();
    // let inputs = vec![vals[0], vals[1]];

    let out = net.network.activate(inputs.clone(), time.dt);
    // genes.fitness += field.get_norm(x, y)[0] * time.dt;
    // genes.fitness -= field.get_norm(x, y)[1] * time.dt * 0.1;

    // energy.amt -= out[0].abs() + out[1].abs() * 0.01 * time.dt;

    // pos.x += (out[0]) * time.dt * 60.;
    // pos.y += (out[1] - 0.5) * time.dt * 60.;

    body.theta += out[0] * time.dt * 1.;
    // body.theta += time.dt * 0.5;
    body.position = body.position.advance(out[1] * time.dt * 3.0, body.theta);
    // body.position = body.position.advance(0.1, body.theta);
    body.emits = vec![0.0, out[2].clamp(0.0, 1.0)];

    // genes.fitness -=
    //     ((out[0] - time.sin(1.0, 0.0)).abs()
    //     + (out[1] - time.sin(1.0, 1.571)).abs())
    //      * time.dt * 0.1;
    // genes.fitness -= ((inputs[0] - 0.5).powi(2) + (inputs[1] - 0.5).powi(2)).sqrt() * time.dt;
    // genes.fitness -= (pos.y).abs();
    // genes.fitness -= (pos.x).abs();
    // genes.fitness += (out[0]).abs();
    // genes.fitness += (out[1]).abs();

    body.theta = body.theta.wrap();
    body.position = body.position.clamp(&config.bounds);
    // *pos = pos.wrap(&config.bounds);
}

#[system(for_each)]
pub fn update_energy(b: &mut Body, g: &mut AntGenetic, #[resource] time: &Time) {
    b.energy.amt -= b.energy.decay * time.dt * (0.2 * random() + 0.8);
    if b.energy.amt < 0.0 {
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
            let fitness = (body.color.r - body.color.g - body.color.b) * 0.001;
            // let fitness = 0.0;
            // let fitness = vec![body.color.r, body.color.g, body.color.b]
            //     .into_iter()
            //     .max_by(|a, b| a.partial_cmp(b).unwrap())
            //     .unwrap() as f64;

            pool.report(
                gen.species_id,
                gen.genome.clone(),
                fitness as f64 + gen.fitness as f64,
            );
            commands.remove(*entity);

            let (id, gene) = pool.next().unwrap();
            commands.push(Creature::new(id, gene, config));
        }
    }
}
