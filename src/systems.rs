use std::sync::Arc;
use std::sync::Mutex;

use crate::breeder::*;
use crate::components::*;
use crate::creature::*;
use crate::field::*;
// use crate::field_render::FieldRenderer;
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
            if body.touches(body2) {
                creatures.push((*entity, body2.energy.amt));
                food.push(*entity2);
            }
        }
    }

    // body.energy.amt += body2.energy.amt;
    // body2.energy.amt = 0.0;

    for (entity, amt) in creatures.into_iter() {
        let (b, g) = <(&mut Body, &mut AntGenetic)>::query()
            .get_mut(world, entity)
            .unwrap();
        // b.energy.amt += amt;
        // g.fitness += 1.0;
    }

    for entity in food.into_iter() {
        let b = <&mut Body>::query().get_mut(world, entity).unwrap();
        b.position = Position::random(&config.bounds);
    }
}

#[system(for_each)]
pub fn update_emitters(
    body: &Body,
    #[resource] time: &Time,
    #[resource] field: &mut FieldArr,
    #[resource] config: &Config,
) {
    if body.emits.len() == 0 {
        return;
    }

    let x = body.position.x / config.bounds.width as f64;
    let y = 1.0 - body.position.y / config.bounds.height as f64;

    field.set(x, y, &body.emits);
}

#[system(for_each)]
pub fn update_networks(
    net: &mut Network,
    genes: &mut AntGenetic,
    body: &mut Body,
    #[resource] config: &Config,
    #[resource] time: &Time,
    #[resource] field: &mut FieldArr,
) {
    let x = body.position.x / config.bounds.width as f64;
    let y = 1.0 - body.position.y / config.bounds.height as f64;

    let (cx, cy) = field.get_dx(x, y);
    // let inputs = vec![ cx, cy, 0.0]
        // if (5.0 * time.elapsed).sin() > 0.75 {
        //     1.0 } else { 0.0
        // }, 0.0, 0.0];
    let inputs = vec![cx.r, cy.r, 0.0];
    // let inputs = vec![x - 0.5, y - 0.5];
    let (out, state) = net.network.activate(inputs.clone(), net.state.clone(), time.dt);
    net.state = state;

    // body.color.r = field.get(x, y).r;
    body.color.r = inputs[0];
    // body.color.g = ((5.0 * time.elapsed).sin() + 1.) / 2.;

    // body.theta += out[0] * time.dt * 1.;
    // body.theta += time.dt * 0.5;
    // body.position = body.position.advance(out[1] * time.dt * 0.1, body.theta);
    body.position.x += out[0] * time.dt;
    body.position.y += out[1] * time.dt;

    // let emit = Color {
    //     r: out[2].clamp(0.0, 1.0) * 10.0,
    //     g: out[3].clamp(0.0, 1.0) * 10.0,
    //     b: out[4].clamp(0.0, 1.0) * 10.0,
    //     a: 1.0
    // };
    // body.emits = vec![0.0, body.color.g * 0.5, body.color.b * 0.5, 1.0];
    body.emits = body.color.into();
    genes.fitness += field.get(x, y).r;
    // genes.fitness -= (out[0] - (5.0 * time.elapsed).cos()).powi(2);
    // genes.fitness -= (out[1] - (5.0 * time.elapsed).sin()).powi(2);

    // genes.fitness += body.color.r;
    // genes.fitness -= (x - 0.5).powi(2) + (y - 0.5).powi(2);

    body.theta = body.theta.wrap();
    body.position = body.position.clamp(&config.bounds);
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
            let fitness = random() * 0.00000001;
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
