use crate::components::{Body, Color, Energy, Position};
use legion::*;
use tui::style;
use tui::widgets::canvas::{Context, Painter, Shape};

impl From<Color> for style::Color {
    fn from(c: Color) -> style::Color {
        style::Color::Rgb(
            (c.a * c.r * 255.).floor() as u8,
            (c.a * c.b * 255.).floor() as u8,
            (c.a * c.g * 255.).floor() as u8,
        )
    }
}

impl Shape for Position {
    fn draw(&self, painter: &mut Painter) {
        if let Some((ux, uy)) = painter.get_point(self.x, self.y) {
            painter.paint(ux, uy, style::Color::White);
        }
    }
}

struct PositionBody(Position, Body);
impl From<(&Position, &Body)> for PositionBody {
    fn from(x: (&Position, &Body)) -> Self {
        Self(*x.0, *x.1)
    }
}

impl Shape for PositionBody {
    fn draw(&self, painter: &mut Painter) {
        if let Some((ux, uy)) = painter.get_point(self.0.x, self.0.y) {
            painter.paint(ux, uy, self.1.color.into());
        }
    }
}

pub fn render(ctx: &mut Context, world: &World, _resources: &Resources) {
    // this time we have &Velocity and &mut Position
    let mut query = <(&Position, &Body)>::query();
    for pos in query.iter(world) {
        ctx.draw(&PositionBody(*pos.0, *pos.1));
    }
}

