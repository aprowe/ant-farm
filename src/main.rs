mod app;
mod breeder;
mod field_render;
mod components;
mod creature;
mod field;
mod prelude;
mod resources;
mod systems;
mod utils;
mod render;

use std::sync::Arc;
use std::sync::Mutex;

use app::App;
use crate::render::*;

use glium::Surface as _;
use glium::glutin;

fn main() {
    // Set up Event Loops
    let event_loop = glutin::event_loop::EventLoop::new();

    // Build Window
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("egui_glium example");

    // Set up Context
    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    // Combine into a display
    let mut display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Create app
    let mut app = App::new();

    // Create renderer
    let renderer = AppRenderable::new(&app, &display);
    let field = field::Field::new(10, 10, vec![0.0], vec![0.0], ndarray::array![
        [0.0, 1.0, 0.0]
    ]);

    // Field Renderer
    let field_render = Arc::new(Mutex::new(field_render::FieldRenderer::default()));
    let mut frender = field_render.lock().unwrap().render(&display);

    app.resources.insert(field_render.clone());

    // Speed of sim
    let sim_speed = 1;

    let mut last_update = std::time::Instant::now();

    // Main Event Loops
    event_loop.run(move |event, _, control_flow| {
        // Schedule next frame
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_secs_f64(0.016);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle Window Events
        match event {
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            glutin::event::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                        return;
                    }

                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == glutin::event::ElementState::Pressed {
                            if let Some(key) = input.virtual_keycode {
                                match key {
                                    glutin::event::VirtualKeyCode::C => {}
                                    glutin::event::VirtualKeyCode::L => {}
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => return
                }
            }

            _ => return,
        }

        // Update Frame time
        let now = std::time::Instant::now();
        let dt: f64 = (now - last_update).as_secs_f64();
        last_update = now;

        // Update all
        for i in 0..sim_speed {
            app.update(dt);
        }

        // Clear Display
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        use evo::utils::{random, random_i};

        let x = random_i(900);
        let y = random_i(900);
        for i in 0..10 {
            field_render.lock().unwrap().set(x + i, y + i,prelude::Color::rgb(random(), random(), 0.0));
        }

        frender(&mut target);
        renderer.render(&app, &mut target);
        target.finish().unwrap();
    });
}
