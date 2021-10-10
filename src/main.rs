mod components;
mod render;
mod resources;
mod systems;
mod breeder;
mod creature;
#[allow(dead_code)]
mod utils;

use crate::components::*;
use crate::resources::*;
use crate::systems::*;
use crate::creature::*;
use crate::utils::{Config as EventConfig, Event, Events};
use breeder::AntBreeder;
use evo::Pool;
use evo::pool::Ratios;
use legion::*;
use std::{error::Error, io, time::Duration};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{
        canvas::{Canvas, Map, MapResolution, Painter, Rectangle, Shape},
        Block, Borders,
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut world = World::default();
    let mut resources = Resources::default();
    let mut pool = Pool::new(200, AntBreeder::default());
    pool.ratios = Ratios::<f32> {
        top: 0.1,
        mutate: 0.4,
        cross: 0.4,
        random: 0.1,
    };

    let bounds = Rect::new(0, 0, 100, 100);
    let config = Config { bounds };

    // Add Creatures' components
    for (id, g) in (&mut pool).take(50) {
        world.push(Creature::new(id, g, &config).components());
    }

    resources.insert(config);
    resources.insert(pool);

    let mut schedule = Schedule::builder()
        .add_system(update_networks_system())
        .add_system(update_energy_system())
        .add_system(remove_dead_system())
        .build();

    // Setup event handlers
    let time = Time{ dt: 0.1, elapsed: 0.0 };
    let config = EventConfig {
        tick_rate: Duration::from_millis((time.dt * 1000.0) as u64),
        ..Default::default()
    };
    let events = Events::with_config(config);
    let mut speed = 1;


    loop {
        terminal.draw(|f| {
            let canvas = Canvas::default()
                .marker(tui::symbols::Marker::Dot)
                .paint(|ctx| {
                    render::render(ctx, &world, &resources);
                })
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0]);
            f.render_widget(canvas, f.size());
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('1') => {
                    speed = 1;
                    eprintln!("Speed: {}", speed);
                }
                Key::Char('2') => {
                    speed = 10;
                    eprintln!("Speed: {}", speed);
                }
                Key::Char('3') => {
                    speed = 100;
                    eprintln!("Speed: {}", speed);
                }
                Key::Char('4') => {
                    speed = 1000;
                    eprintln!("Speed: {}", speed);
                }
                _ => {}
            },
            Event::Tick => {
                resources.insert(time.tick());

                for i in 0..speed {
                    schedule.execute(&mut world, &mut resources);
                }
            }
        }
    }

    Ok(())
}
