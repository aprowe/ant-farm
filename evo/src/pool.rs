use maplit::hashmap;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::breeder::Breeder;
use crate::utils::*;

pub struct Species<B: Breeder> {
    pub model: B::Genome,
    pub champion: Option<(f32, B::Genome)>,
    pub pool: Vec<B::Genome>,
    pub reported: Vec<(f32, B::Genome)>,
    pub gens_empty: i32,
    pub last_mean: f32,
}

impl<B: Breeder> Species<B> {
    pub fn new(model: B::Genome) -> Self {
        Self {
            model,
            gens_empty: 0,
            pool: vec![],
            reported: vec![],
            champion: None,
            last_mean: 0.0,
        }
    }

    pub fn from_breeder(breeder: &B) -> Self {
        Self::new(breeder.random())
    }

    pub fn choose_model(&mut self) {
        if self.reported.len() == 0 {
            return;
        }

        // Sort scores
        self.reported.sort_by_key(|x| (-100000.0 * x.0) as i32);
        // record mean
        self.last_mean = self.reported.iter().map(|x| x.0).sum::<f32>() / self.reported.len() as f32;
        // Record champion
        self.champion = Some(self.reported.first().unwrap().clone());
        self.model = self.reported.sample().clone().1;

//         let top = self
//             .reported
//             .iter()
//             .take(10)
//             .map(|(s, _)| *s)
//             .collect::<Vec<f32>>();

        // if let Some((_, model)) = self.reported.first().cloned() {
        // } else {
        //     unreachable!();
        // }
    }

    /// Generate the next generatio
    pub fn next_generation(
        &self,
        breeder: &B,
        global: &[&B::Genome],
        size: i32,
        ratios: &Ratios<f32>,
    ) -> Vec<B::Genome> {
        let size = std::cmp::max(1, size);
        if self.reported.len() == 0 {
            return vec![];
        }
        // let size = std::cmp::max(2, (self.reported.len() as f32 * growth).round() as usize);

        // Get normalized ratio valies
        let ratios = if size > 2 {
            ratios.to_int(size as usize)
        } else {
            Ratios {
                top: 1,
                mutate: 1,
                cross: 0,
                random: 0,
            }
        };

        // Start new pool with the top talent
        let mut top: Vec<_> = self
            .reported
            .iter()
            .take(std::cmp::max(ratios.top, 1))
            .cloned()
            .collect();

        // Fill a new Pool
        let mut new_pool = Vec::fill(ratios.random, || breeder.random());

        // Create cross breeds
        for _ in 0..ratios.cross {
            let a = top.sample();
            let b = if random() < 0.05 {
                (0.0, (*Vec::from(global).sample()).clone())
            } else {
                top.sample().clone()
            };


            if a.0 > b.0 {
                new_pool.push(breeder.breed(&a.1, &b.1));
            } else {
                new_pool.push(breeder.breed(&b.1, &a.1));
            }
        }

        for _ in 0..ratios.mutate {
            let a = top.sample();
            new_pool.push(breeder.mutate(&a.1));
        }

        // Add top to pool
        new_pool.append(&mut top.drain(..).map(|a| a.1).collect());
        new_pool
    }
}

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
    pool: Vec<(i32, B::Genome)>,
    reported: Vec<(i32, B::Genome, f32)>,
    species: HashMap<i32, Species<B>>,
    cur_id: i32,

    // Size of gene pool
    size: usize,

    /// Ratios of different methods
    pub ratios: Ratios<f32>,

    /// Stats
    pub mean_score: f32,
    pub generations: i32,
    pub last_mean: f32,
    pub last_best: f32,
    pub champion: Option<(f32, B::Genome)>,
    pub gens_without_improvement: i32,
}

/// Make Pool an iterator
impl<B> Iterator for Pool<B>
where
    B: Breeder,
{
    type Item = (i32, B::Genome);

    ///
    /// Get Next Gene in pool
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.pool.is_empty() {
            self.next_generation();
        }

        self.pool.pop()
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
            pool: Vec::<(i32, B::Genome)>::fill(size as usize, || (0, breeder.random())),
            cur_id: 1,
            species: hashmap! {
                0 => Species::from_breeder(&breeder),
            },
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

    /// Report a Gene back to the pool
    pub fn report<F>(&mut self, species_id: i32, member: F, score: f32) -> bool
    where
        F: Into<B::Genome>,
    {
        if let Some(s) = self.species.get_mut(&species_id) {
            s.reported.push((score, member.into()));
        } else {
            self.species
                .insert(self.cur_id, Species::new(member.into()));
            self.cur_id += 1;
        }

        if self.pool.len() == 0 {
            self.next_generation();
            return true;
        }

        false
    }

    fn next<F>(&mut self) -> (i32, F)
    where
        F: From<B::Genome>,
    {
        if self.pool.is_empty() {
            self.next_generation();
        }
        let tup = self.pool.pop().unwrap();
        (tup.0, tup.1.into())
    }

    /// Calculate the Running Mean score
    pub fn mean_score(&self) -> f32 {
        if self.species.len() == 0 {
            panic!("No Species!");
        }

        self.species
            .iter()
            .map(|(_, s)| s.last_mean)
            .sum::<f32>()
            / self.species.len() as f32
    }

    /// Calculate the Running Mean score
    pub fn mean_best(&self) -> f32 {
        if self.species.len() == 0 {
            panic!("No Species!");
        }

        let scores: Vec<_> = self
            .species
            .values()
            .filter(|s| s.champion.is_some())
            .map(|s| s.champion.as_ref().unwrap().0)
            .collect();

        scores.sum() / scores.len() as f32
    }

    /// Calculate the Running Mean score
    pub fn get_champion(&self) -> Option<(f32, B::Genome)> {
        if self.species.len() == 0 {
            return None;
        }

        let mut best: Option<&(f32, B::Genome)> = None;
        for c in self
            .species
            .values()
            .filter(|s| s.champion.is_some())
            .map(|s| s.champion.as_ref().unwrap())
        {
            if best.is_none() || c.0 > best.unwrap().0 {
                best = Some(&c);
            }
        }
        best.cloned()
    }

    /// Generate the next generatio
    pub fn next_generation(&mut self) {
        // Sort and Choose model for each species
        self.species.values_mut().for_each(|s| {
            s.choose_model();

            if s.reported.len() == 0 {
                s.gens_empty += 1;
            } else {
                s.gens_empty = 0;
            }
        });

        // Keep species less than 3 empty generations
        self.species.retain(|_, s| s.gens_empty == 0);

        // Calculate last mean
        self.last_mean = self.mean_score();

        let champion = self.get_champion();
        if self.champion.is_none()
            || (champion.is_some()
                && champion.as_ref().unwrap().0 > self.champion.as_ref().unwrap().0)
        {
            self.champion = champion;
            self.gens_without_improvement = 0;
        } else {
            self.gens_without_improvement += 1;
        }

        eprintln!("Generation {}:", self.generations);
        eprintln!("\tMean = {}", self.last_mean);
        eprintln!("\tMean Best = {}", self.mean_best());
        eprintln!(
            "\tBest Score= {}",
            self.champion.as_ref().map(|c| c.0).unwrap_or(-9999.9)
        );
        eprintln!("\tSpecies = {}", self.species.len());

        // Get refrence for all models
        let combined_pool: Vec<&B::Genome> = self
            .species
            .iter()
            .flat_map(|(_, s)| s.reported.iter().map(|(_, s)| s))
            .take(self.size)
            .collect();

        eprintln!("\tReported = {}", combined_pool.len());

        // Evolve And Sort Each Species
        let mut new_pool: Vec<(Option<i32>, B::Genome)> = self.species.values()
            // Flatten and gather new genomes
            .flat_map(|s| {
                let size = s.last_mean * (self.size as f32) / self.last_mean / self.species.len() as f32;
                s.next_generation(&self.breeder, &combined_pool, size.round() as i32, &self.ratios)
            })
            // Iterator over all genomes
            .map(|g| {
                (
                    // Find a matching Species (Option<i32>)
                    self.species
                        .iter()
                        .find(|(_, s)| self.breeder.is_same(&s.model, &g))
                        // Get its ID if it exists
                        .map(|s| *s.0),
                    // Pair it with a genome
                    g,
                )
            })
            .collect();

        new_pool.shuffle();

        // Resolve new IDs
        let mut new_pool: Vec<(i32, B::Genome)> = new_pool
            .into_iter()
            .take(self.size + (self.size as f32 * self.species.len() as f32 * 0.1) as usize)
            .map(|(id, g)| {
                if let Some(id) = id {
                    (id, g)
                } else {
                    self.cur_id += 1;
                    self.species.insert(self.cur_id, Species::new(g.clone()));
                    // Fix New Species
                    (self.cur_id, g)
                }
            })
            .collect();

        // for (id, s) in self.species.iter().filter(|(_, s)| s.champion.is_some()) {
        //     new_pool.push((*id, s.champion.as_ref().unwrap().clone().1));
        // }

        eprintln!("\tOrganisms = {}", new_pool.len());

        // eprintln!("Cleaning up");
        // Clears its reported genes
        self.species.values_mut().for_each(|s| {
            s.reported = vec![];
        });

        self.pool = new_pool;
        self.mean_score = self.mean_score();
        self.reported = vec![];
        self.generations += 1;
    }

    pub fn run<F, G>(
        &mut self,
        max_gens: i32,
        max_gens_without_improve: i32,
        f: F,
    ) -> (bool, f32, G)
    where
        F: Fn(G) -> f32,
        G: From<B::Genome>,
    {
        let mut t1 = SystemTime::now();
        while self.generations < max_gens
            && self.gens_without_improvement < max_gens_without_improve
        {
            // let now = SystemTime::now().duration_since(t1).unwrap();
            // eprintln!("{:?}", now);
            // t1 = SystemTime::now();
            let (id, g): (i32, B::Genome) = self.next();
            let score = f(g.clone().into());
            self.report(id, g, score);
        }

        let converged = self.gens_without_improvement >= max_gens_without_improve;

        eprintln!(
            "Done after {} Generations, Score: {}, Converged: {}",
            self.generations,
            self.champion.as_ref().unwrap().0,
            converged
        );

        let champ = self.champion.as_ref().cloned().unwrap();
        (converged, champ.0, champ.1.into())
    }
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

#[cfg(test)]
mod test {
    use super::*;

    use crate::breeder::FloatBreeder;

    #[test]
    fn test_species() {
        let f = FloatBreeder::default();
        let mut s = Species::from_breeder(&f);
        s.pool.push(FloatBreeder::default().random());
    }

    struct Fuck {
        x: f32,
    }

    impl From<f32> for Fuck {
        fn from(x: f32) -> Self {
            Self { x }
        }
    }

    #[test]
    fn test_pool() {
        let f = FloatBreeder::default();
        let mut pool = Pool::new(100, f);
        for i in 0..10000 {
            let (id, g): (_, Fuck) = pool.next();
            pool.report(id, g.x, g.x.abs());
        }
    }

    #[test]
    fn test_run() {
        let f = FloatBreeder::default();
        let mut pool = Pool::new(100, f);
        let result = pool.run(100, 5, |g: Fuck| g.x.abs());
        assert!(result.2.x.abs() > 0.999);
    }
}
