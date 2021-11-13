use crate::breeder::*;
use crate::creature::*;
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
            mutate: 0.45,
            cross: 0.43,
            random: 0.02,
        };

        // Config
        let bounds = Rect::new(0, 0, 100, 100);
        let config = Config { bounds };

        // Add Creatures' components
        for (id, g) in (&mut pool).take(30) {
            world.push(Creature::new(id, g, &config));
        }

        for _ in 0..40 {
            world.push(Food::new(vec![30.0, 0.0, 0.0], &config));
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
        resources.insert(time);

        // Set up Update Schedule
        let mut schedule = Schedule::builder()
            .add_system(update_emitters_system())
            .add_system(detect_system())
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

        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn handle_event(&mut self, e: AppEvent) {
        match e {

        }
    }
}

pub enum AppEvent {
}
