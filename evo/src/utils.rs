
pub fn random() -> f32 {
    rand::random()
}

pub fn random_d(delta: f32) -> f32 {
    (rand::random::<f32>() - 0.5) * 2.0 * delta
}

pub fn random_clamp(min: f32, max: f32) -> f32 {
    rand::random::<f32>() * (max - min) + min
}

pub fn random_i(max: usize) -> usize {
    if max == 0 {
        return 0;
    }
    rand::random::<usize>() % max
}

pub fn clamp<T>(num: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd<T>,
{
    match num {
        x if x < min => min,
        x if x > max => max,
        x => x,
    }
}


pub trait VecUtils {
    type Item;

    fn append_with<F>(&mut self, size: usize, f: F)
    where
        F: FnMut() -> Self::Item;
    fn fill<F>(size: usize, f: F) -> Vec<Self::Item>
    where
        F: FnMut() -> Self::Item;
    fn sample(&self) -> &Self::Item;
    fn shuffle(&mut self);
}

impl<T> VecUtils for Vec<T> {
    type Item = T;

    fn fill<F>(size: usize, f: F) -> Vec<Self::Item>
    where
        F: FnMut() -> Self::Item,
    {
        let mut v = Vec::<T>::new();
        v.append_with(size, f);
        v
    }

    fn append_with<F>(&mut self, size: usize, mut f: F)
    where
        F: FnMut() -> T,
    {
        self.append(&mut (0..size).map(|_| f().into()).collect::<Vec<T>>());
    }

    fn sample(&self) -> &Self::Item {
        if self.len() == 0 {
            panic!("Sample on Empty Vec");
        }
        unsafe { self.get_unchecked(random_i(self.len()) as usize) }
    }

    fn shuffle(&mut self) {
        let mut v: Vec<T> = self.drain(0..).collect();
        for _ in 0..v.len() {
            self.push(v.remove(random_i(v.len())));
        }
    }
}

pub trait Sum<T> {
    fn sum(&self) -> T;
    fn mean(&self) -> T;
}

impl Sum<f32> for Vec<f32> {
    fn sum(&self) -> f32 {
        self.iter().fold(0.0, |a, b| a + b)
    }
    fn mean(&self) -> f32 {
        self.sum() / self.len() as f32
    }
}

#[macro_export]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        assert!(($x - $y).abs() < $d);
    };
}
