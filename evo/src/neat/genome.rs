use super::gene::Gene;
use super::mutation::Mutation;
use crate::{breeder::Breeder, utils::random};
use rand::Rng;

// use mutation::Mutation;
use rand;
use rand::distributions::*;
use std::cmp;

/// Vector of Genes
/// Holds a count of last neuron added, similar to Innovation number
#[derive(Default, Debug, Clone)]
pub struct NeatGenome {
    pub genes: Vec<Gene>,
    pub last_neuron_id: usize,
}

impl NeatGenome {
    ///Add initial input and output neurons interconnected
    pub fn new_initialized(input_neurons: usize, output_neurons: usize) -> NeatGenome {
        let mut genome = NeatGenome::default();
        for i in 0..input_neurons {
            for o in 0..output_neurons {
                genome.add_gene(Gene::new_connection(i, input_neurons + o));
            }
        }
        genome
    }

    /// Get vector of all genes in this genome
    pub fn get_genes(&self) -> &Vec<Gene> {
        &self.genes
    }

    /// Number of genes
    pub fn len(&self) -> usize {
        self.last_neuron_id + 1 // first neuron id is 0
    }
    /// is genome empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn mutate_add_connection(&mut self) {
        let mut rng = rand::thread_rng();
        let neuron_ids_to_connect = {
            if self.last_neuron_id == 0 {
                vec![0, 0]
            } else {
                (&mut rng)
                    .sample_iter(Uniform::new_inclusive(0, self.last_neuron_id))
                    .take(2)
                    .collect()
                // rand::seq::sample_iter(&mut rng, 0..self.last_neuron_id + 1, 2).unwrap()
            }
        };
        self.add_connection(neuron_ids_to_connect[0], neuron_ids_to_connect[1]);
    }

    pub fn mutate_connection_weight(&mut self, probability: f64) {
        for gene in &mut self.genes {
            <dyn Mutation>::connection_weight(gene, rand::random::<f64>() < probability);
        }
    }

    pub fn mutate_toggle_expression(&mut self) {
        let mut rng = rand::thread_rng();
        let selected_gene: usize = (&mut rng)
            .sample_iter(Uniform::new(0, self.genes.len()))
            .take(1)
            .last()
            .unwrap();
        <dyn Mutation>::toggle_expression(&mut self.genes[selected_gene]);
    }

    pub fn mutate_bias(&mut self, probability: f64) {
        let mut rng = rand::thread_rng();

        let selected_gene: usize = (&mut rng)
            .sample_iter(Uniform::new(0, self.genes.len()))
            .take(1)
            .last()
            .unwrap();
        <dyn Mutation>::connection_bias(&mut self.genes[selected_gene], rand::random::<f64>() < probability);
    }

    pub fn mutate_add_neuron(&mut self) {
        let (gene1, gene2) = {
            let mut rng = rand::thread_rng();
            let selected_gene: usize = (&mut rng)
                .sample_iter(Uniform::new(0, self.genes.len()))
                .take(1)
                .last()
                .unwrap();
            let gene = &mut self.genes[selected_gene];
            self.last_neuron_id += 1;
            <dyn Mutation>::add_neuron(gene, self.last_neuron_id)
        };
        self.add_gene(gene1);
        self.add_gene(gene2);
    }

    pub fn add_connection(&mut self, in_neuron_id: usize, out_neuron_id: usize) {
        let gene = <dyn Mutation>::add_connection(in_neuron_id, out_neuron_id);
        self.add_gene(gene);
    }

    /// Add a new gene and checks if is allowd. Only can connect next neuron or already connected
    /// neurons.
    pub fn add_gene(&mut self, gene: Gene) {
        let max_neuron_id = self.last_neuron_id + 1;

        if gene.in_neuron_id == gene.out_neuron_id && gene.in_neuron_id > max_neuron_id {
            panic!(
                "Try to create a gene neuron unconnected, max neuron id {}, {} -> {}",
                max_neuron_id,
                gene.in_neuron_id,
                gene.out_neuron_id
            );
        }

        if gene.in_neuron_id > self.last_neuron_id {
            self.last_neuron_id = gene.in_neuron_id;
        }
        if gene.out_neuron_id > self.last_neuron_id {
            self.last_neuron_id = gene.out_neuron_id;
        }
        match self.genes.binary_search(&gene) {
            Ok(pos) => self.genes[pos].toggle(),
            Err(_) => self.genes.push(gene),
        }
        self.genes.sort();
    }

    /// Total weigths of all genes
    pub fn total_weights(&self) -> f64 {
        let mut total = 0f64;
        for gene in &self.genes {
            total += gene.weight;
        }
        total
    }

    /// Total num genes
    // TODO len() is enough
    pub fn total_genes(&self) -> usize {
        self.genes.len()
    }

    // http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf - Pag. 110
    // I have considered disjoint and excess genes as the same
    pub fn compatibility_distance(&self, other: &NeatGenome) -> f64 {
        // TODO: optimize this method
        let c2 = 1f64;
        let c3 = 0.2f64;

        // Number of excess
        let n1 = self.genes.len();
        let n2 = other.genes.len();
        let n = cmp::max(n1, n2);

        if n == 0 {
            return 0f64; // no genes in any genome, the genomes are equal
        }

        let matching_genes = self
            .genes
            .iter()
            .filter(|i1_gene| other.genes.contains(i1_gene))
            .collect::<Vec<&Gene>>();
        let n3 = matching_genes.len();

        // Disjoint / excess genes
        let d = n1 + n2 - (2 * n3);

        // average weight differences of matching genes
        let mut w = matching_genes.iter().fold(0f64, |acc, &m_gene| {
            acc + (m_gene.weight
                - &other.genes[other.genes.binary_search(m_gene).unwrap()].weight)
                .abs()
        });

        // if no matching genes then are completely different
        w = if n3 == 0 { 1f64 } else { w / n3 as f64 };

        // compatibility distance
        (c2 * d as f64 / n as f64) + c3 * w
    }
}
