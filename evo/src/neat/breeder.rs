use super::genome::NeatGenome;
use crate::utils::random;
use crate::breeder::Breeder;
use rand;

// Defaults
// const MUTATE_CONNECTION_WEIGHT: f64 = 0.90f64;
// const MUTATE_ADD_CONNECTION: f64 = 0.005f64;
// const MUTATE_ADD_NEURON: f64 = 0.004f64;
// const MUTATE_TOGGLE_EXPRESSION: f64 = 0.001f64;
// const MUTATE_CONNECTION_WEIGHT_PERTURBED_PROBABILITY: f64 = 0.90f64;
// const MUTATE_TOGGLE_BIAS: f64 = 0.01;

pub struct NeatBreeder {
    pub inputs: usize,
    pub outputs: usize,
    pub mutate_connection_weight: f64,
    pub mutate_add_connection: f64,
    pub mutate_add_neuron: f64,
    pub mutate_toggle_expression: f64,
    pub mutate_perturb_prob: f64,
    pub mutate_connection_bias: f64,
}

impl NeatBreeder {
    pub fn new(inputs: usize, outputs: usize) -> Self {
        Self {
            inputs,
            outputs,
            ..Self::default()
        }
    }
}

impl Default for NeatBreeder {
    fn default() -> Self {
        Self {
            inputs: 2,
            outputs: 2,
            mutate_connection_weight: 0.9f64,
            mutate_connection_bias: 0.5,
            mutate_add_connection: 0.02f64,
            mutate_add_neuron: 0.02f64,
            mutate_toggle_expression: 0.02f64,
            mutate_perturb_prob: 0.9f64,
        }
    }
}

impl Breeder for NeatBreeder {
    type Genome = NeatGenome;

    fn mutate(&self, gene: &Self::Genome) -> Self::Genome {
        let mut gene = gene.clone();
        if (random() as f64) < self.mutate_add_connection || gene.genes.is_empty() {
            gene.mutate_add_connection();
        };

        if (random() as f64) < self.mutate_add_neuron {
            gene.mutate_add_neuron();
        };

        if (random() as f64) < self.mutate_connection_weight {
            gene.mutate_connection_weight(self.mutate_perturb_prob);
        };

        if (random() as f64) < self.mutate_toggle_expression {
            gene.mutate_toggle_expression();
        };

        if (random() as f64) < self.mutate_connection_bias {
            gene.mutate_bias(self.mutate_perturb_prob);
        };
        gene
    }

    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome {
        let mut genome = NeatGenome::default();
        for gene in &gene1.genes {
            genome.add_gene({
                //Only mate half of the genes randomly
                if rand::random::<f64>() > 0.5f64 {
                    *gene
                } else {
                    match gene2.genes.binary_search(gene) {
                        Ok(position) => gene2.genes[position],
                        Err(_) => *gene,
                    }
                }
            });
        }
        genome
    }
    fn random(&self) -> Self::Genome {
        let mut  g = Self::Genome::new_initialized(self.inputs, self.outputs);
        {0..5}.for_each(|_|g = self.mutate(&g));
        g
    }
    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
        // gene1.is_same_specie(gene2)
        gene1.compatibility_distance(gene2) < 1.0f64
    }
}

