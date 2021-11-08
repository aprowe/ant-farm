use crate::breeder::*;
use crate::creature::*;
use crate::field::Field;
use crate::prelude::*;
use crate::systems::*;
use evo::pool::Ratios;

pub struct App {
    pub world: World,
    pub resources: Resources,
    pub schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        // Create World
        let mut world = World::default();

        // Instantiate resources
        // Pool
        let mut pool = Pool::new(200, AntBreeder::default());
        pool.ratios = Ratios::<f64> {
            top: 0.1,
            mutate: 0.4,
            cross: 0.4,
            random: 0.1,
        };

        // Config
        let bounds = Rect::new(0, 0, 100, 100);
        let config = Config { bounds };

        // Pheromone Field
        let mut field = Field::new(
            100,
            100,
            vec![0.99, 0.95],
            vec![0.8, 0.99],
            array![[0.0, 1.0, 0.0], [1.0, 0.0, 0.0]], //[1.0, 0.0, 1.0],],
        );

        // field.set(30, 30, vec![1.0, 100.0, 0.0]);
        // field.set(60, 60, vec![0.0, 0.0, 10.0]);

        // Add Creatures' components
        for (id, g) in (&mut pool).take(400) {
            world.push(Creature::new(id, g, &config));
        }

        for _ in 0..20 {
            world.push(Food::new(vec![1.0, 0.0], &config));
        }

        // Setup event handlers
        let mut time = Time {
            dt: 0.1,
            elapsed: 0.0,
        };

        // Create resources
        let mut resources = Resources::default();
        resources.insert(config);
        resources.insert(pool);
        resources.insert(field);
        resources.insert(time);

        // Set up Update Schedule
        let mut schedule = Schedule::builder()
            .add_system(update_emitters_system())
            // .add_system(detect_system())
            .add_system(update_networks_system())
            .add_system(update_energy_system())
            .add_system(remove_dead_system())
            .build();

        Self {
            world,
            resources,
            schedule,
        }
    }

    pub fn update(&mut self, dt: f64) {
        // update time
        if let Some(mut time) = self.resources.get_mut::<Time>() {
            time.elapsed += dt;
        }

        // update field
        if let Some(mut field) = self.resources.get_mut::<Field>() {
            field.update(dt);
        }

        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn handle_event(&mut self, e: AppEvent) {
        match e {

        }
    }
}

pub enum AppEvent {
}
