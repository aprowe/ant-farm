use std::error::Error;
use std::time::Duration;

// #[macro_use]
// extern crate rusty_dashed;

use crate::breeder::*;
use crate::creature::*;
use crate::events::*;
use crate::field::Field;
use crate::prelude::*;
use crate::render;
use crate::systems::*;
use crate::widgets::*;
use evo::pool::Ratios;
use termion::event::Key;
use termion::raw::IntoRawMode;

// Primary APp Run Method
pub fn run() -> Result<(), Box<dyn Error>> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = tui::backend::TermionBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
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

    // Event Handler
    let mut events = TermEventHandler::new(Duration::from_millis(100));

    // Current Speed
    let mut speed = 1;

    loop {
        terminal.draw(|f| {
            // Draw Main Canvas
            let canvas = tui::widgets::canvas::Canvas::default()
                // .marker(tui::symbols::Marker::Dot)
                .paint(|ctx| {
                    render::render(ctx, &world, &resources);
                })
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0]);
            f.render_widget(canvas, f.size());

            // Side Canvas
            let canvas = tui::widgets::canvas::Canvas::default()
                // .marker(tui::symbols::Marker::Dot)
                .paint(|ctx| {
                    let mut query = <&Body>::query();
                    if let Some(body) = query.iter(&world).next() {
                        ctx.draw(&render::Circle {
                            x: 0.3 + 0.14 * body.theta.cos(),
                            y: 0.3 + 0.14 * body.theta.sin(),
                            r: 0.1,
                            c: tui::style::Color::Yellow,
                        });
                        ctx.draw(&render::Circle {
                            x: 0.3,
                            y: 0.3,
                            r: 0.24,
                            c: tui::style::Color::Cyan,
                        });
                    }
                })
                .x_bounds([0.0, 1.0])
                .y_bounds([0.0, 1.0]);
            f.render_widget(canvas, Rect::new(0, 0, 40, 20));

            if let Some(field) = resources.get::<Field>() {
                f.render_widget(FieldWidget::from(&(*field)), f.size());
            }
        })?;

        // Handle Events
        match events.next() {
            TermEvent::Input(input) => match input {
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
            TermEvent::Tick => {
                // Update Time
                time.elapsed += time.dt;
                resources.insert(time);

                for _ in 0..speed {
                    // update field
                    if let Some(mut field) = resources.get_mut::<Field>() {
                        field.update(time.dt);
                    }
                    schedule.execute(&mut world, &mut resources);
                }
            }
        }
    }
    Ok(())
}
