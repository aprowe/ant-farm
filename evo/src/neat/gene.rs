extern crate rand;

// use rand::Closed01;
use crate::utils::*;
use std::cmp::Ordering;

/// A connection Gene
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "telemetry", derive(Serialize))]
pub struct Gene {
    pub in_neuron_id: usize,
    pub out_neuron_id: usize,
    pub weight: f64,
    pub enabled: bool,
    pub bias: f64,
}

impl Eq for Gene {}

impl PartialEq for Gene {
    fn eq(&self, other: &Gene) -> bool {
        self.in_neuron_id == other.in_neuron_id && self.out_neuron_id == other.out_neuron_id
    }
}

impl Ord for Gene {
    fn cmp(&self, other: &Gene) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.in_neuron_id == other.in_neuron_id {
            if self.out_neuron_id > other.out_neuron_id {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if self.in_neuron_id > other.in_neuron_id {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Gene {
    fn partial_cmp(&self, other: &Gene) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Gene {
    /// Create a new gene
    pub fn new(
        in_neuron_id: usize,
        out_neuron_id: usize,
        weight: f64,
        enabled: bool,
        bias: f64,
    ) -> Gene {
        Gene {
            in_neuron_id,
            out_neuron_id,
            weight,
            enabled,
            bias,
        }
    }

    /// Create a new gene with a specific connection
    pub fn new_connection(in_neuron_id: usize, out_neuron_id: usize) -> Gene {
        Gene {
            in_neuron_id,
            out_neuron_id,
            ..Gene::default()
        }
    }

    /// Generate a weight
    pub fn generate_weight() -> f64 {
        random_d(1.0) as f64
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

impl Default for Gene {
    fn default() -> Gene {
        Gene {
            in_neuron_id: 1,
            out_neuron_id: 1,
            weight: Gene::generate_weight(),
            enabled: true,
            bias: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(n_in: usize, n_out: usize) -> Gene {
        Gene {
            in_neuron_id: n_in,
            out_neuron_id: n_out,
            ..Gene::default()
        }
    }

    #[test]
    fn should_be_able_to_binary_search_for_a_gene() {
        let mut genome = vec![g(0, 1), g(0, 2), g(3, 2), g(2, 3), g(1, 5)];
        genome.sort();
        genome.binary_search(&g(0, 1)).unwrap();
        genome.binary_search(&g(0, 2)).unwrap();
        genome.binary_search(&g(1, 5)).unwrap();
        genome.binary_search(&g(2, 3)).unwrap();
        genome.binary_search(&g(3, 2)).unwrap();
    }
}
