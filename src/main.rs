mod app;
mod breeder;
mod components;
mod creature;
mod field;
mod prelude;
mod resources;
mod systems;
mod utils;
mod render;
mod executor;

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

    // Field Renderer
    let mut field = field::Field::new(&display);
    app.resources.insert(field.to_arr());

    // Speed of sim
    let mut sim_speed = 1;

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
                                    glutin::event::VirtualKeyCode::Key1 => {
                                        sim_speed = 1;
                                    }
                                    glutin::event::VirtualKeyCode::Key2 => {
                                        sim_speed = 10;
                                    }
                                    glutin::event::VirtualKeyCode::Key3 => {
                                        sim_speed = 50;
                                    }
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
            field.update();

            // Update app
            app.update(0.016);

            // Update Field
            if let Some(mut arr) = app.resources.get_mut::<field::FieldArr>() {
                field.update_arr(&mut arr);
            }
        }

        // Clear Display
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Update / Render Field
        field.render(&mut target);
        renderer.render(&app, &mut target);
        target.finish().unwrap();
    });
}
