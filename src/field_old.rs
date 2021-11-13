use ndarray::prelude::*;

#[derive(Clone)]
pub struct Field {
    pub w: usize,
    pub h: usize,
    pub slots: usize,
    pub colors: Array<f64, Dim<[usize; 2]>>,
    pub decay: Array<f64, Dim<[usize; 1]>>,
    pub disperse: Array<f64, Dim<[usize; 1]>>,
    pub data: ndarray::Array<f64, Dim<[usize; 3]>>,
    pub derivative: ndarray::Array<f64, Dim<[usize; 4]>>,
}

impl Field {
    pub fn new(
        w: usize,
        h: usize,
        decay: Vec<f64>,
        disperse: Vec<f64>,
        colors: Array<f64, Dim<[usize; 2]>>,
    ) -> Self {
        let slots = colors.shape()[0];
        Self {
            w,
            h,
            slots,
            colors,
            decay: Array::from_vec(decay),
            disperse: Array::from_vec(disperse),
            data: Array::zeros((h, w, slots)),
            derivative: Array::zeros((h, w, slots, 2)),
        }
    }
}

impl Field {
    /// Convert a slice to a tui Color
    pub fn to_color(c: &[f64]) -> (u8, u8, u8) {
        (
            ((c[0] * 255.0) as u8).clamp(0, 255),
            ((c[1] * 255.0) as u8).clamp(0, 255),
            ((c[2] * 255.0) as u8).clamp(0, 255),
        )
    }

    pub fn update(&mut self, _dt: f64) {
        let disperse = &self.disperse;
        let decay = &self.decay;

        // Created padded version
        let mut padded: Array<f64, _> = Array::zeros((self.h + 2, self.w + 2, self.slots));
        padded.slice_mut(s![1..-1, 1..-1, ..]).assign(&self.data);

        // take slices for each direction
        let left = padded.slice(s![..-2, 1..-1, ..]);
        let right = padded.slice(s![2.., 1..-1, ..]);
        let up = padded.slice(s![1..-1, ..-2, ..]);
        let down = padded.slice(s![1..-1, 2.., ..]);

        // Next iteration
        let new = (&left + &right + &up + &down) / 4.0 * (1.0 - disperse) + &self.data * disperse;
        // Assign it
        (new * decay).assign_to(&mut self.data);

        // Calculate derivative
        self.derivative.slice_mut(s![.., .., .., 0]).assign(&(&left - &right));
        self.derivative.slice_mut(s![.., .., .., 1]).assign(&(&up - &down));
    }

    /// Set a specific cell to a value
    pub fn set(&mut self, row: usize, col: usize, slots: Vec<f64>) {
        let cells = self.data.slice_mut(s![row, col, ..]);
        &Array::from_vec(slots).assign_to(cells);
    }

    /// Add to a cell
    pub fn add(&mut self, row: usize, col: usize, slots: Vec<f64>) {
        let mut cell = self.data.slice_mut(s![row, col, ..]);
        cell += &Array::from_vec(slots);
    }

    /// Normalized Add to a cell
    pub fn add_norm(&mut self, x: f64, y: f64, slots: Vec<f64>) {
        let col = (x * (self.w - 1) as f64).round() as usize;
        let row = (y * (self.h - 1) as f64).round() as usize;
        self.add(row, col, slots);
    }

    /// Get data from a row and column
    pub fn get(&self, row: usize, col: usize) -> Vec<f64> {
        self.data.slice(s![row, col, ..]).as_slice().unwrap().into()
    }

    pub fn get_norm(&self, x: f64, y: f64) -> Vec<f64> {
        let col = (x * (self.w - 1) as f64).round() as usize;
        let row = (y * (self.h - 1) as f64).round() as usize;
        self.get(row, col)
    }

    pub fn get_derivative(&self, x: f64, y: f64) -> Vec<(f64, f64)> {
        let col = (x * (self.w - 1) as f64).round() as usize;
        let row = (y * (self.h - 1) as f64).round() as usize;
        self.derivative.slice(s![row, col, .., ..]).outer_iter().map(|pair| {
            let slice = pair.as_slice().unwrap();
            (slice[0], slice[1])
        }).collect()
    }

    /// get a normalized color location for graphics
    pub fn get_normalized_col(&self, x: f64, y: f64) -> (u8, u8, u8) {
        let col = (x * (self.w - 1) as f64).round() as usize;
        let row = (y * (self.h - 1) as f64).round() as usize;

        // Get the slots
        let entry = self.data.slice(s![row, col, ..]);

        // Convert slots to colors
        let color = entry.dot(&self.colors);

        // Convert to u8
        Field::to_color(color.as_slice().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_set() {
        let mut field = Field::new(
            10,
            10,
            vec![0.99; 2],
            vec![0.9; 2],
            array![[1.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
        );

        field.set(1, 2, vec![1.0, 0.1]);

        let r = field.get(1, 2);
        assert_eq!(r[0], 1.0);
        assert_eq!(r[1], 0.1);
    }

    #[test]
    fn test_add() {
        let mut field = Field::new(
            10,
            10,
            vec![0.99; 2],
            vec![0.9; 2],
            array![[1.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
        );

        {
            field.add(0, 1, vec![0.1, 0.2]);
            field.add(0, 1, vec![0.1, 0.2]);

            let r = field.get(0, 1);
            assert_eq!(r[0], 0.2);
            assert_eq!(r[1], 0.4);
        }

        {
            // Normalized version
            field.add_norm(0.4, 0.3, vec![0.1, 0.2]);
            field.add_norm(0.4, 0.3, vec![0.1, 0.2]);

            let r = field.get_norm(0.4, 0.3);
            assert_eq!(r[0], 0.2);
            assert_eq!(r[1], 0.4);
        }
    }
}
