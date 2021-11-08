use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;
use crate::field::Field;
use crate::render::FnRender;
use crate::render::Vertex;
use glium::pixel_buffer::PixelBuffer;
use glium::program;
use glium::uniform;
use glium::Surface;
use glium::Texture2d;

pub struct FieldRenderer {
    texture: Option<Rc<Texture2d>>,
    buffer: Option<Rc<RefCell<PixelBuffer<(u8, u8, u8, u8)>>>>,
}

impl FnRender for FieldRenderer {
    fn render(&mut self, display: &glium::Display) -> Box<dyn FnMut(&mut glium::Frame)> {
        let vertices = glium::VertexBuffer::new(
            display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                },
            ],
        )
        .unwrap();

        // building the index buffer
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 3, 2, 1],
        )
        .unwrap();

        // Load shaders
        let frag = std::fs::read_to_string("shaders/field_frag.glsl").unwrap();
        let vert = std::fs::read_to_string("shaders/field_vert.glsl").unwrap();

        // compiling shaders and linking them together
        let program = glium::program!(display,
            330 => {
                vertex: &vert,
                fragment: &frag,
            },
        )
        .unwrap();

        let tex = Rc::new(glium::Texture2d::empty(display, 1000, 1000).unwrap());
        let buffer = Rc::new(RefCell::new(tex.read_to_pixel_buffer()));
        self.texture = Some(tex.clone());
        self.buffer = Some(buffer.clone());

        Box::new(move |frame: &mut glium::Frame| {
            *buffer.borrow_mut() = tex.read_to_pixel_buffer();

            tex.as_surface()
                .draw(
                    &vertices,
                    &indices,
                    &program,
                    &uniform! {PASS_NUM: 0, TEX: &*tex},
                    &Default::default(),
                )
                .unwrap();

            frame
                .draw(
                    &vertices,
                    &indices,
                    &program,
                    &uniform! {PASS_NUM: 1, TEX: &*tex},
                    &Default::default(),
                )
                .unwrap();
        })
    }
}

impl Default for FieldRenderer {
    fn default() -> Self {
        Self {
            texture: None,
            buffer: None,
        }
    }
}

impl<'a> glium::texture::Texture2dDataSource<'a>  for Color {
    type Data = u8;


    fn into_raw(self) -> glium::texture::RawImage2d<'a, Self::Data> {
        glium::texture::RawImage2d::from_raw_rgb(vec![
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8
            ], (1, 1))
    }
}

impl FieldRenderer {
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.texture.as_ref().unwrap().write(glium::Rect{
            bottom: y as u32,
            left: x as u32,
            width: 1,
            height: 1,
        }, color);
    }

    pub fn get(&self, x: usize, y: usize) -> [f64; 4] {
        if self.texture.is_none() {
            return [0.0, 0.0, 0.0, 0.0];
        }

        let idx = x + y * self.texture.as_ref().unwrap().width() as usize;
        let pix = self
            .buffer
            .as_ref()
            .unwrap()
            .borrow()
            .slice(idx..idx + 1)
            .unwrap()
            .read()
            .unwrap()[0];

        [
            pix.0 as f64 / 255.0,
            pix.1 as f64 / 255.0,
            pix.2 as f64 / 255.0,
            pix.3 as f64 / 255.0,
        ]
    }
}
