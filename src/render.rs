use crate::{components::Body, utils::*};
use glium::implement_vertex;
use glium::uniform;
use glium::Display;
use glium::*;
use legion::*;
use crate::app::App;
use crate::utils::*;
use glium::index::PrimitiveType;
use glium::{glutin, Surface};

pub struct AppRenderable {
    program: Program,
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
}

impl Renderable<App> for AppRenderable {
    fn new(app: &App, display: &Display) -> Self {
        let (vertex, index) = &circle(1.0, 20);
        let vertices = glium::VertexBuffer::new(
            display,
            &vertex
        )
        .unwrap();

        // building the index buffer
        let indices =
            glium::IndexBuffer::new(display, PrimitiveType::TriangleFan, &index).unwrap();


        // Load shaders
        let frag = std::fs::read_to_string("shaders/default_frag.glsl").unwrap();
        let vert = std::fs::read_to_string("shaders/default_vert.glsl").unwrap();

        // compiling shaders and linking them together
        let program = program!(display,
            330 => {
                vertex: &vert,
                fragment: &frag,
            },
        )
        .unwrap();

        Self {
            program, indices, vertices
        }
    }

    fn render(&self, app: &App, target: &mut Frame) {
        for body in <&Body>::query().iter(&app.world) {

            // building the uniforms
            let uniforms = uniform! {
                scale: 0.01f32,
                theta: body.theta as f32,
                color: [body.color.r as f32, body.color.g as f32, body.color.b as f32],
                pos: [body.position.x as f32 / 100.0, body.position.y as f32 / 100.0],
            };

            target
                .draw(
                    &self.vertices,
                    &self.indices,
                    &self.program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();
        }
    }
}

pub trait Renderable<T> {
    fn new(obj: &T, display: &Display) -> Self;
    fn render(&self, obj: &T, frame: &mut Frame);
}


// Utilities
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    // color: [f32; 3],
}
implement_vertex!(Vertex, position);

pub fn circle(r: f32, v: u16) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertex: Vec<Vertex> = vec![Vertex {
        position: [0.0, 0.0],
    }];

    let mut index: Vec<u16> = vec![];

    for i in 1..=v {
        index.push(0);
        index.push(i);
        index.push(i+1);
    }

    vertex.extend((0..=v).map(|t| {
        let t = t as f32 * std::f32::consts::TAU / v as f32;
        Vertex {
            position: [r*t.cos(), r*t.sin()]
        }
    }));

    (vertex, index)
}

pub trait FnRender {
    fn render(&mut self, display: &Display) -> Box<dyn FnMut(&mut Frame)>;
}
