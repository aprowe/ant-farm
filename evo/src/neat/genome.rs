use super::gene::Gene;
use super::mutation::Mutation;
use crate::utils::random;
use rand::Rng;

// use mutation::Mutation;
use crate::breeder::Breeder;
use rand;
use rand::distributions::*;
use std::cmp;

/// Vector of Genes
/// Holds a count of last neuron added, similar to Innovation number
#[derive(Default, Debug, Clone)]
pub struct NeatGenome {
    genes: Vec<Gene>,
    pub last_neuron_id: usize,
}

// const MUTATE_CONNECTION_WEIGHT: f64 = 0.90f64;
// const MUTATE_ADD_CONNECTION: f64 = 0.005f64;
// const MUTATE_ADD_NEURON: f64 = 0.004f64;
// const MUTATE_TOGGLE_EXPRESSION: f64 = 0.001f64;
// const MUTATE_CONNECTION_WEIGHT_PERTURBED_PROBABILITY: f64 = 0.90f64;
// const MUTATE_TOGGLE_BIAS: f64 = 0.01;

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

    fn mutate_add_connection(&mut self) {
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

    fn mutate_connection_weight(&mut self, probability: f64) {
        for gene in &mut self.genes {
            <dyn Mutation>::connection_weight(gene, rand::random::<f64>() < probability);
        }
    }

    fn mutate_toggle_expression(&mut self) {
        let mut rng = rand::thread_rng();
        let selected_gene: usize = (&mut rng)
            .sample_iter(Uniform::new(0, self.genes.len()))
            .take(1)
            .last()
            .unwrap();
        <dyn Mutation>::toggle_expression(&mut self.genes[selected_gene]);
    }

    fn mutate_toggle_bias(&mut self) {
        let mut rng = rand::thread_rng();
        let selected_gene: usize = (&mut rng)
            .sample_iter(Uniform::new(0, self.genes.len()))
            .take(1)
            .last()
            .unwrap();
        <dyn Mutation>::toggle_bias(&mut self.genes[selected_gene]);
    }

    fn mutate_add_neuron(&mut self) {
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

    fn add_connection(&mut self, in_neuron_id: usize, out_neuron_id: usize) {
        let gene = <dyn Mutation>::add_connection(in_neuron_id, out_neuron_id);
        self.add_gene(gene);
    }

    /// Add a new gene and checks if is allowd. Only can connect next neuron or already connected
    /// neurons.
    pub fn add_gene(&mut self, gene: Gene) {
        let max_neuron_id = self.last_neuron_id + 1;

        if gene.in_neuron_id() == gene.out_neuron_id() && gene.in_neuron_id() > max_neuron_id {
            panic!(
                "Try to create a gene neuron unconnected, max neuron id {}, {} -> {}",
                max_neuron_id,
                gene.in_neuron_id(),
                gene.out_neuron_id()
            );
        }

        if gene.in_neuron_id() > self.last_neuron_id {
            self.last_neuron_id = gene.in_neuron_id();
        }
        if gene.out_neuron_id() > self.last_neuron_id {
            self.last_neuron_id = gene.out_neuron_id();
        }
        match self.genes.binary_search(&gene) {
            Ok(pos) => self.genes[pos].set_enabled(),
            Err(_) => self.genes.push(gene),
        }
        self.genes.sort();
    }

    /// Total weigths of all genes
    pub fn total_weights(&self) -> f64 {
        let mut total = 0f64;
        for gene in &self.genes {
            total += gene.weight();
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
    fn compatibility_distance(&self, other: &NeatGenome) -> f64 {
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
            acc + (m_gene.weight()
                - &other.genes[other.genes.binary_search(m_gene).unwrap()].weight())
                .abs()
        });

        // if no matching genes then are completely different
        w = if n3 == 0 { 1f64 } else { w / n3 as f64 };

        // compatibility distance
        (c2 * d as f64 / n as f64) + c3 * w
    }
}

pub struct NeatBreeder {
    inputs: usize,
    outputs: usize,
    mutate_connection_weight: f64,
    mutate_add_connection: f64,
    mutate_add_neuron: f64,
    mutate_toggle_expression: f64,
    mutate_connection_weight_perturbed_probability: f64,
    mutate_toggle_bias: f64,
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
            mutate_connection_weight: 0.5f64,
            mutate_add_connection: 0.015f64,
            mutate_add_neuron: 0.014f64,
            mutate_toggle_expression: 0.05f64,
            mutate_connection_weight_perturbed_probability: 0.5f64,
            mutate_toggle_bias: 0.05,
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
            gene.mutate_connection_weight(self.mutate_connection_weight_perturbed_probability);
        };

        if (random() as f64) < self.mutate_toggle_expression {
            gene.mutate_toggle_expression();
        };

        if (random() as f64) < self.mutate_toggle_bias {
            gene.mutate_toggle_bias();
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
        Self::Genome::new_initialized(self.inputs, self.outputs)
    }
    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
        // gene1.is_same_specie(gene2)
        gene1.compatibility_distance(gene2) < 1.0f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neat::organism::NeatNetwork;
    use crate::pool::Pool;
    use std::f64::EPSILON;

    #[test]
    fn breeder() {
        let mut pool = Pool::new(10, NeatBreeder::default());

        for i in 0..100 {
            let (id, g) = pool.next().unwrap();
            let mut o = NeatNetwork::new(g.clone());
            let input = vec![0.0, 0.0];
            let mut output = vec![0.0, 0.0];
            o.activate(input, &mut output);
            pool.report(id, g, output.iter().sum::<f64>() as f32);
        }
        let (id, g) = pool.next().unwrap();
        let mut o = NeatNetwork::new(g.clone());
        let input = vec![0.0, 0.0];
        let mut output = vec![0.0, 0.0];
        o.activate(input, &mut output);
        println!("{:?}", output);
    }

    #[test]
    fn mutation_connection_weight() {
        let mut genome = NeatGenome::default();
        genome.add_gene(Gene::new(0, 0, 1f64, true, false));
        let orig_gene = genome.genes[0];
        genome.mutate_connection_weight(0.9);
        // These should not be same size
        assert!((genome.genes[0].weight() - orig_gene.weight()).abs() > EPSILON);
    }

    #[test]
    fn mutation_add_connection() {
        let mut genome = NeatGenome::default();
        genome.add_connection(1, 2);

        assert!(genome.genes[0].in_neuron_id() == 1);
        assert!(genome.genes[0].out_neuron_id() == 2);
    }

    #[test]
    fn mutation_add_neuron() {
        let mut genome = NeatGenome::default();
        genome.mutate_add_connection();
        genome.mutate_add_neuron();
        assert!(!genome.genes[0].enabled());
        assert!(genome.genes[1].in_neuron_id() == genome.genes[0].in_neuron_id());
        assert!(genome.genes[1].out_neuron_id() == 1);
        assert!(genome.genes[2].in_neuron_id() == 1);
        assert!(genome.genes[2].out_neuron_id() == genome.genes[0].out_neuron_id());
    }

    #[test]
    #[should_panic(expected = "Try to create a gene neuron unconnected, max neuron id 1, 2 -> 2")]
    fn try_to_inject_a_unconnected_neuron_gene_should_panic() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(2, 2, 0.5f64, true, false));
    }

    #[test]
    fn two_genomes_without_differences_should_be_in_same_specie() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 0, 1f64, true, false));
        genome1.add_gene(Gene::new(0, 1, 1f64, true, false));
        let mut genome2 = NeatGenome::default();
        genome2.add_gene(Gene::new(0, 0, 0f64, true, false));
        genome2.add_gene(Gene::new(0, 1, 0f64, true, false));
        genome2.add_gene(Gene::new(0, 2, 0f64, true, false));
        let b = NeatBreeder::default();
        assert!(b.is_same(&genome1, &genome2));
    }

    #[test]
    fn two_genomes_with_enought_difference_should_be_in_different_species() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 0, 1f64, true, false));
        genome1.add_gene(Gene::new(0, 1, 1f64, true, false));
        let mut genome2 = NeatGenome::default();
        genome2.add_gene(Gene::new(0, 0, 5f64, true, false));
        genome2.add_gene(Gene::new(0, 1, 5f64, true, false));
        genome2.add_gene(Gene::new(0, 2, 1f64, true, false));
        genome2.add_gene(Gene::new(0, 3, 1f64, true, false));
        let b = NeatBreeder::default();
        assert!(!b.is_same(&genome1, &genome2));
    }

    #[test]
    fn already_existing_gene_should_be_not_duplicated() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 0, 1f64, true, false));
        genome1.add_connection(0, 0);
        assert_eq!(genome1.genes.len(), 1);
        assert!((genome1.get_genes()[0].weight() - 1f64).abs() < EPSILON);
    }

    #[test]
    fn adding_an_existing_gene_disabled_should_enable_original() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 1, 0f64, true, false));
        genome1.mutate_add_neuron();
        assert!(!genome1.genes[0].enabled());
        assert!(genome1.genes.len() == 3);
        genome1.add_connection(0, 1);
        assert!(genome1.genes[0].enabled());
        assert!((genome1.genes[0].weight() - 0f64).abs() < EPSILON);
        assert_eq!(genome1.genes.len(), 3);
    }

    #[test]
    fn genomes_with_same_genes_with_little_differences_on_weight_should_be_in_same_specie() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 0, 16f64, true, false));
        let mut genome2 = NeatGenome::default();
        genome2.add_gene(Gene::new(0, 0, 16.1f64, true, false));
        let b = NeatBreeder::default();
        assert!(b.is_same(&genome1, &genome2));
    }

    #[test]
    fn genomes_with_same_genes_with_big_differences_on_weight_should_be_in_other_specie() {
        let mut genome1 = NeatGenome::default();
        genome1.add_gene(Gene::new(0, 0, 5f64, true, false));
        let mut genome2 = NeatGenome::default();
        genome2.add_gene(Gene::new(0, 0, 15f64, true, false));
        let b = NeatBreeder::default();
        assert!(!b.is_same(&genome1, &genome2));
    }

    #[test]
    fn genomes_initialized_has_correct_neurons() {
        let genome1 = NeatGenome::new_initialized(2, 3);
        assert_eq!(genome1.total_genes(), 6);
        assert_eq!(genome1.genes[0].in_neuron_id(), 0);
        assert_eq!(genome1.genes[0].out_neuron_id(), 2);
        assert_eq!(genome1.genes[1].in_neuron_id(), 0);
        assert_eq!(genome1.genes[1].out_neuron_id(), 3);
        assert_eq!(genome1.genes[2].in_neuron_id(), 0);
        assert_eq!(genome1.genes[2].out_neuron_id(), 4);
        assert_eq!(genome1.genes[3].in_neuron_id(), 1);
        assert_eq!(genome1.genes[3].out_neuron_id(), 2);
        assert_eq!(genome1.genes[4].in_neuron_id(), 1);
        assert_eq!(genome1.genes[4].out_neuron_id(), 3);
        assert_eq!(genome1.genes[5].in_neuron_id(), 1);
        assert_eq!(genome1.genes[5].out_neuron_id(), 4);
    }
}
