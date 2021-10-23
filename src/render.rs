use crate::{components::Body, utils::*};
use legion::*;
use tui::style;
use tui::widgets::canvas::{Context, Painter, Shape};

//////////////////////////////////
/// Primary Render Function
///
pub fn render(ctx: &mut Context, world: &World, _resources: &Resources) {
    // this time we have &Velocity and &mut Position
    let mut query = <&Body>::query();
    let mut highlight = true;
    for body in query.iter(world) {
        ctx.draw(&BodyShape { body, highlight });
        highlight = false;
    }
}

//////////////////////////////////
/// Body Shape
///
struct BodyShape<'a> {
    body: &'a Body,
    highlight: bool,
}

impl<'a> From<&'a Body> for BodyShape<'a> {
    fn from(body: &'a Body) -> Self {
        Self {
            body,
            highlight: false,
        }
    }
}

impl<'a> Shape for BodyShape<'a> {
    fn draw(&self, painter: &mut Painter) {
        if let Some((ux, uy)) = painter.get_point(self.body.position.x, self.body.position.y) {
            if self.highlight {
                painter.paint(ux, uy - 1, style::Color::LightBlue);
            }

            painter.paint(ux, uy, Color::from(self.body.theta).into());
        }
    }
}


//////////////////////////////////
/// Circle
///
pub struct Circle {
    pub r: f64,
    pub x: f64,
    pub y: f64,
    pub c: tui::style::Color,
}

impl Shape for Circle {
    fn draw(&self, painter: &mut Painter) {
        let n = 100;

        for i in 0..n {
            let x = i as f64 / n as f64 * std::f64::consts::TAU;
            if let Some((ux, uy)) =
                painter.get_point(self.x + self.r * x.cos(), self.y + self.r * x.sin())
            {
                painter.paint(ux, uy, self.c);
            }
        }
    }
}

//////////////////////////////////
/// Misc rendering
///
impl Shape for Position {
    fn draw(&self, painter: &mut Painter) {
        if let Some((ux, uy)) = painter.get_point(self.x, self.y) {
            painter.paint(ux, uy, style::Color::White);
        }
    }
}

