use super::ctrnn::{Ctrnn, CtrnnNeuralNetwork};
use super::genome::NeatGenome;

/// An network is a NeatGenome with fitness.
/// Also maitain a fitenss measure of the network
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct NeatNetwork {
    pub genome: NeatGenome,
    pub species: i32,
}


impl NeatNetwork {
    /// Create a new organmism form a single genome.
    pub fn new(genome: NeatGenome) -> NeatNetwork {
        NeatNetwork {
            species: -1,
            genome,
        }
    }

    pub fn random(inputs: usize, outputs: usize) -> NeatNetwork {
        NeatNetwork {
            species: -1,
            genome: NeatGenome::new_initialized(inputs, outputs),
        }
    }

    /// Activate this network in the NN
    pub fn activate(&self, mut inputs: Vec<f64>, dt: f64) -> Vec<f64> {
        let in_len = inputs.len();

        // Make same length as
        inputs.truncate(self.genome.len());
        inputs.extend(vec![0.0; self.genome.len() - inputs.len()]);

        let activations = Ctrnn::default().activate_nn(
            dt,
            dt / 2.0,

            &CtrnnNeuralNetwork {
                //current state of neuron(j)
                y: &vec![0.0; self.genome.len()],
                //τ - time constant ( t > 0 ). The neuron's speed of response to an external sensory signal. Membrane resistance time.
                tau: &vec![0.01; self.genome.len()],
                //w - weights of the connection from neuron(j) to neuron(i)
                wji: &self.get_weights(),
                //θ - bias of the neuron(j)
                theta: &self.get_bias(),
                //I - external input to neuron(i)
                i: &inputs,
            }
        );

        activations.split_at(in_len).1.to_vec()
    }

    fn get_weights(&self) -> Vec<f64> {
        let neurons_len = self.genome.len();
        let mut matrix = vec![0.0; neurons_len * neurons_len];
        for gene in self.genome.get_genes() {
            if gene.enabled() {
                matrix[(gene.out_neuron_id() * neurons_len) + gene.in_neuron_id()] = gene.weight()
            }
        }
        matrix
    }

    fn get_bias(&self) -> Vec<f64> {
        let neurons_len = self.genome.len();
        let mut matrix = vec![0.0; neurons_len];
        for gene in self.genome.get_genes() {
            if gene.is_bias() {
                matrix[gene.in_neuron_id()] += 1f64;
            }
        }
        matrix
    }
}

/// Convert from a tuple
impl From<(i32, NeatGenome)> for NeatNetwork {
    fn from((species, genome): (i32, NeatGenome)) -> Self {
        Self {
            species,
            genome
        }
    }
}

impl From<NeatGenome> for NeatNetwork {
    fn from(g: NeatGenome) -> Self {
        Self::new(g)
    }
}

impl From<NeatNetwork> for NeatGenome {
    fn from(val: NeatNetwork) -> Self {
        val.genome
    }
}

