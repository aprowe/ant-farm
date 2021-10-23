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
    }
}
