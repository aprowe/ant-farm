use statrs::distribution::{Normal, ContinuousCDF};

use crate::breeder::Breeder;
use crate::utils::*;

///
/// Main Pool Struct
///
pub struct Pool<B>
where
    B: Breeder,
{
    /// Breeder instance
    breeder: B,

    /// Mutable Pooles
    // pool: Vec<(i32, B::Genome)>,
    reported: Vec<(i32, B::Genome, f64)>,
    // species: HashMap<i32, Species<B>>,

    // Size of gene pool
    size: usize,

    /// Ratios of different methods
    pub ratios: Ratios<f64>,

    /// Stats
    pub mean_score: f64,
    pub generations: i32,
    pub last_mean: f64,
    pub last_best: f64,
    pub champion: Option<(f64, B::Genome)>,
    pub gens_without_improvement: i32,
}

impl<B> Iterator for Pool<B>
where
    B: Breeder,
{
    type Item = (i32, B::Genome);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next().into())
    }
}

/// Primary Pool implementation
impl<B> Pool<B>
where
    B: Breeder,
{
    /// New pool with size  and breeder
    pub fn new(size: usize, breeder: B) -> Self {
        Self {
            size,
            // pool: vec![],
            // species: hashmap! {
            //     0 => Species::from_breeder(&breeder),
            // },
            breeder,
            ratios: Ratios {
                top: 0.05,
                random: 0.05,
                cross: 0.45,
                mutate: 0.45,
            },
            mean_score: 0.0,
            generations: 0,
            reported: vec![],
            last_mean: -9999.0,
            last_best: -9999.0,
            champion: None,
            gens_without_improvement: 0,
        }
    }

    fn next<F>(&mut self) -> (i32, F)
    where
        F: From<B::Genome>,
    {
        if self.reported.len() <= 1 {
            return (0, self.breeder.random().into());
        }

        // Let Pool Fill up
        let x = self.reported.len() as f64 / self.size as f64 * 10.0;
        if random() > x {
            return (0, self.breeder.random().into());
        }

        let cum = self.ratios.cumulative();
        let next = match random() {
            x if x < cum.top => {
                // dbg!("Top");
                self.reported.sample_weighted(3).1.clone()
            }
            x if x < cum.mutate => {
                // dbg!("Mutate");
                self.breeder.mutate(&self.reported.sample().1)
            }
            x if x < cum.cross => {
                // dbg!("Cross");

                let g1 = self.reported.sample();
                let g2 = self.reported.sample();

                if g1.2 > g2.2 {
                    self.breeder.breed(&g1.1, &g2.1)
                } else {
                    self.breeder.breed(&g2.1, &g1.1)
                }
            }
            _ => {
                // dbg!("Random");
                self.breeder.random()
            }
        };

        (0, next.into())
    }

    // fn report(&mut self, score: f64, gene: B::Genome) {
    pub fn report<F>(&mut self, species_id: i32, genome: F, score: f64) -> bool
    where
        F: Into<B::Genome>,
    {
        self.reported.push((0, genome.into(), score));
        self.cull_weak();
        true
    }

    fn cull_weak(&mut self) {
        if self.reported.len() <= 10 {
            return;
        }
        // Get Scores and stats on score
        let scores: Vec<_> = self.reported.iter().map(|r| r.2).collect();

        // Create normal distribution
        let (mean, std) = scores.std();
        let stats = Normal::new(mean, std).unwrap();
        eprintln!("{}, {}", mean, std);

        // Sort scores
        self.reported.sort_by(|a, b| {
            a.2.partial_cmp(&b.2).unwrap()
        });

        // Probability Coeffictient
        let mut y = self.reported.len() as f64 / self.size as f64;
        let dy = 1.0 / self.size as f64;
        let mut current_size = self.reported.len() as f64;
        let orig_size = current_size;
        let max_size = self.size as f64;

        // Probabilities
        let len = self.reported.len();
        self.reported.retain(|r|{
            // Get the Cumulated Distribution probability
            let x = stats.cdf(r.2);

            // Transform it with how full the pool is
            let x = transform_prob(x, current_size / max_size);
            if random() / orig_size > x {
                current_size -= 1.0;
                return false;
            }
            true
        });

        eprintln!("{} => {}", len, self.reported.len())
    }
}

/// Transform probability based on a second factor
/// x:0-1 main probility
/// f(x, y=0) => 1
/// f(x, y=0.5) => x
/// f(x, y=1) => 0
fn transform_prob(x: f64, y: f64) -> f64{
    let a = 1.0 - 1.0/y;
    a * x / (a*x + (1.0-x) / a)
}

/// Define ratio of different breed strategies
/// Templated for easy convertion
#[derive(Debug, Clone)]
pub struct Ratios<T>
where
    T: Clone + Copy,
{
    pub top: T,
    pub mutate: T,
    pub cross: T,
    pub random: T,
}

impl Ratios<f32> {
    /// Convert into integer ratios
    fn to_int(&self, max: usize) -> Ratios<usize> {
        let top = (max as f32 * self.top).floor() as usize;
        let mutate = (max as f32 * self.mutate).floor() as usize;
        let cross = (max as f32 * self.cross).floor() as usize;
        Ratios {
            top,
            mutate,
            cross,
            random: max - top - mutate - cross,
        }
    }

}

impl<T> Ratios<T> where T: std::ops::Add<T, Output=T> + Copy + Clone {
    fn cumulative(&self) -> Self {
        Self {
            top: self.top,
            mutate: self.top + self.mutate,
            cross: self.top + self.mutate + self.cross,
            random: self.top + self.mutate + self.cross + self.random,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::breeder::FloatBreeder;

    #[test]
    fn test_new_pool() {
        let mut pool = Pool::new(100, FloatBreeder::default());
        for i in 0..100 {
            let f = i as f64;
            let (_, f) : (_, f64) = pool.next();
            pool.report(0, f, random());
        }

        // pool.cull_weak();
    }

    #[test]
    fn print_prob() {
        let mut vec: Vec<_> = vec![];
        let stats = Normal::new(0.5, 0.25).unwrap();
        for i in 0..100 {
            let f = i as f64 / 100.0;
            let x = stats.cdf(f);
            vec.push((f, x,
                    transform_prob(x, 0.25),
                    transform_prob(x, 0.50),
                    transform_prob(x, 0.75),
                    ))
        }

        dbg!(vec);
    }

}
