use std::borrow::Cow;

use crate::prelude::*;
use crate::render::Vertex;
use glium::{pixel_buffer::PixelBuffer, program, uniform, Surface, Texture2d};

const QUAD: [Vertex; 4] = [
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
];

pub struct Field {
    texture: Texture2d,
    buffer: PixelBuffer<(u8, u8, u8, u8)>,
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u16>,
    program: glium::Program,
}


impl Field {
    pub fn new(display: &glium::Display) -> Self {
        let vertices = glium::VertexBuffer::new(display, &QUAD).unwrap();

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

        let texture = glium::Texture2d::empty_with_format(display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, 1000, 1000).unwrap();
        let buffer = texture.read_to_pixel_buffer();

        Field {
            texture,
            buffer,
            vertices,
            indices,
            program,
        }
    }

    pub fn update(&mut self) {
        self.texture
            .as_surface()
            .draw(
                &self.vertices,
                &self.indices,
                &self.program,
                &uniform! {PASS_NUM: 0, TEX: &self.texture},
                &Default::default(),
            )
            .unwrap();
        self.buffer = self.texture.read_to_pixel_buffer();
    }

    pub fn render(&mut self, frame: &mut glium::Frame) {
        frame
            .draw(
                &self.vertices,
                &self.indices,
                &self.program,
                &uniform! {PASS_NUM: 1, TEX: &self.texture},
                &Default::default(),
            )
            .unwrap();
    }

    pub fn set<C: Into<Color>>(&mut self, x: usize, y: usize, color: C) {
        self.texture.write(
            glium::Rect {
                bottom: y as u32,
                left: x as u32,
                width: 1,
                height: 1,
            },
            color.into(),
        );
    }

    pub fn get<C: From<Color>>(&self, x: usize, y: usize) -> C {
        let idx = x + y * self.texture.width() as usize;
        let pix = self.buffer.slice(idx..idx + 1).unwrap().read().unwrap()[0];

        Color::from(pix).into()
    }

    pub fn set_norm<C: Into<Color>>(&mut self, x: f64, y: f64, color: C) {
        let left = (x * (self.texture.width() - 1) as f64).round() as u32;
        let bottom = (y * (self.texture.width() - 1) as f64).round() as u32;

        self.texture.write(
            glium::Rect {
                bottom,
                left,
                width: 1,
                height: 1,
            },
            color.into(),
        );
    }

    pub fn to_arr(&self) -> FieldArr {
        let pix = self.buffer.slice(..).unwrap().read().unwrap();
        let w = self.texture.width() as usize;
        let h = self.texture.height() as usize;
        FieldArr::new(
            Array::<(u8, u8, u8, u8), _>::from_shape_vec((h, w), pix).unwrap()
        )
    }

    pub fn update_arr(&mut self, field: &mut FieldArr) {
        for (x, y, c) in field.queue.drain(..) {
            self.set_norm(x, y, c);
        }

        let pix = self.buffer.slice(..).unwrap().read().unwrap();
        let w = self.texture.width() as usize;
        let h = self.texture.height() as usize;

        field.data = Array::<(u8, u8, u8, u8), _>::from_shape_vec((h, w), pix).unwrap();
    }

    pub fn from_arr(&mut self, field: &mut FieldArr) {
        for (x, y, c) in field.queue.drain(..) {
            self.set_norm(x, y, c);
        }
    }
}

pub struct FieldArr {
    pub data: Array<(u8, u8, u8, u8), Dim<[usize; 2]>>,
    pub queue: Vec<(f64, f64, Color)>
}

impl FieldArr {
    pub fn new(data: Array<(u8, u8, u8, u8), Dim<[usize; 2]>>) -> Self {
        Self {
            data,
            queue: Vec::new()
        }
    }

    pub fn get(&self, x: f64, y: f64) -> Color {
        let x = x.clamp(0.0, 1.0);
        let y = 1.0 - y.clamp(0.0, 1.0);

        let y = (y * (self.data.shape()[0] - 1) as f64).round() as usize;
        let x = (x * (self.data.shape()[1] - 1) as f64).round() as usize;
        let pix = self.data[(y, x)];

        Color::from(pix)
    }

    pub fn get_dx(&self, x: f64, y: f64) -> (Color, Color) {
        let dy = 1.0 / self.data.shape()[0] as f64;
        let dx = 1.0 / self.data.shape()[1] as f64;


        let cx = self.get(x + dx, y) - self.get(x - dx, y);
        let cy = self.get(x, y + dy) - self.get(x, y - dy);

        return (cx, cy);
    }

    pub fn set<C: Into<Color>>(&mut self, x: f64, y: f64, c: C) {
        let x = x.clamp(0.0, 0.99999);
        let y = 1.0 - y.clamp(0.0, 0.99999);

        self.queue.push((x, y, c.into()))
    }

}

impl<'a> glium::texture::Texture2dDataSource<'a> for Color {
    type Data = u8;

    fn into_raw(self) -> glium::texture::RawImage2d<'a, Self::Data> {
        glium::texture::RawImage2d::from_raw_rgb(
            vec![
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8,
            ],
            (1, 1),
        )
    }
}

