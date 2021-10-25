use tui::widgets::Widget;
use crate::field::Field;

pub struct FieldWidget<'a> {
    pub field: &'a Field
}

impl<'a> From<&'a Field> for FieldWidget<'a> {
    fn from(field: &'a Field) -> Self {
        Self {
            field
        }
    }
}

impl<'a> Widget for FieldWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        for x in 0..area.width {
            for y in 0..area.height {
                let fx = x as f64 / area.width as f64;
                let fy = y as f64 / area.height as f64;
                // let fy1 = (2*y) as f64 / (area.height*2) as f64;
                // let fy2 = (2*y+1) as f64 / (area.height*2) as f64;

                let c = self.field.get_normalized_col(fx, fy);
                // let c1 = self.field.get_normalized_col(fx, fy1);
                // let c2 = self.field.get_normalized_col(fx, fy2);

                let cell = buf.get_mut(x, y);
                // cell.set_symbol("▄");
                cell.set_bg(tui::style::Color::Rgb(c.0, c.1, c.2));
                // cell.set_fg(tui::style::Color::Rgb(c2.0, c2.1, c2.2));
            }
        }
    }
}
