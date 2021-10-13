use crate::utils::*;
use std::collections::HashMap;
use std::fmt::Debug;

/// Breeder trait {{{1
/// Provides methods to create new and mix Genes
pub trait Breeder {
    type Genome: Clone + Debug;

    fn mutate(&self, gene: &Self::Genome) -> Self::Genome;
    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome;
    fn random(&self) -> Self::Genome;
    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool;
}

/// VecBreeder {{{1
/// Breeder that breeds lists of floats
pub struct VecBreeder {
    pub size: usize,
    pub min: f64,
    pub max: f64,
    pub delta: f64,
    pub mutate_rate: f64,
    pub flip_rate: f64,
    pub is_same_threshold: f64,
}

impl Default for VecBreeder {
    fn default() -> Self {
        VecBreeder {
            size: 100,
            min: 0.0,
            max: 1.0,
            delta: 0.5,
            mutate_rate: 2.0,
            flip_rate: 1.0,
            is_same_threshold: 0.5
        }
    }
}

impl Breeder for VecBreeder {
    type Genome = Vec<f64>;

    fn mutate(&self, gene: &Self::Genome) -> Self::Genome {
        gene.iter()
            .map(|x| {
                if random() < self.mutate_rate / (self.size as f64) {
                    clamp(x + random_d(self.delta), self.min, self.max)
                } else {
                    *x
                }
            })
            .collect()
    }

    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome {
        let mut flip = false;
        gene1
            .iter()
            .zip(gene2)
            .map(|(g1, g2)| {
                if random() < self.flip_rate / (self.size as f64) {
                    flip = !flip;
                }
                if flip {
                    *g1
                } else {
                    *g2
                }
            })
            .collect()
    }

    fn random(&self) -> Self::Genome {
        (0..self.size)
            .map(|_| random_clamp(self.min, self.max))
            .collect()
    }

    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
        gene1.iter().zip(gene2).fold(0f64, |s, (x, y)| {
            s + (x - y) * (x - y) / (self.max - self.min) / (self.max - self.min)
        }) / (self.size as f64)
            < self.is_same_threshold
    }
}
/// End VecBreeder

/// Float Breeder
pub struct FloatBreeder {
    pub min: f64,
    pub max: f64,
    pub delta: f64,
}

impl Breeder for FloatBreeder {
    type Genome = f64;

    fn mutate(&self, gene: &Self::Genome) -> Self::Genome {
        (gene + random_d(self.delta)).clamp(self.min, self.max)
    }

    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome {
        let r = random();
        gene1 * r + gene2 * (1. - r)
    }

    fn random(&self) -> Self::Genome {
        random_clamp(self.min, self.max)
    }

    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
        (gene1 - gene2).abs() < self.delta * 2.0
    }
}

impl Default for FloatBreeder {
    fn default() -> Self {
        Self {
            min: -1.0,
            max: 1.0,
            delta: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_macros::derive_breeder;

    #[derive_breeder]
    struct MyBreeder {
        #[breeder(0.5)]
        f: FloatBreeder,

        #[breeder(0.5)]
        v: VecBreeder,
    }

    #[test]
    fn derive() {
        let m = MyBreeder {
            f: FloatBreeder::default(),
            v: VecBreeder::default(),
        };
    }
}
