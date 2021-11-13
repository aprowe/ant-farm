use super::gene::Gene;

pub trait Mutation {}

impl dyn Mutation {
    pub fn connection_weight(gene: &mut Gene, perturbation: bool) {
        let mut new_weight = Gene::generate_weight();
        if perturbation {
            new_weight += gene.weight;
        }
        gene.weight = new_weight;
    }

    pub fn connection_bias(gene: &mut Gene, perturbation: bool) {
        let mut new_bias = Gene::generate_weight();
        if perturbation {
            new_bias += gene.bias;
        }
        gene.bias = new_bias;
    }

    pub fn add_connection(in_neuron_id: usize, out_neuron_id: usize) -> Gene {
        Gene::new(
            in_neuron_id,
            out_neuron_id,
            Gene::generate_weight(),
            true,
            0.0,
        )
    }

    pub fn add_neuron(gene: &mut Gene, new_neuron_id: usize) -> (Gene, Gene) {
        gene.enabled = false;

        let gen1 = Gene::new(gene.in_neuron_id, new_neuron_id, 1f64, true, 0.0);

        let gen2 = Gene::new(
            new_neuron_id,
            gene.out_neuron_id,
            gene.weight,
            true,
            0.0,
        );
        (gen1, gen2)
    }

    pub fn toggle_expression(gene: &mut Gene) {
        if gene.enabled {
            gene.enabled = false
        } else {
            gene.enabled = true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neat::gene::Gene;

    #[test]
    fn mutate_toggle_gene_should_toggle() {
        let mut gene = Gene::new(0, 1, 1f64, false, 0.0);

        <dyn Mutation>::toggle_expression(&mut gene);
        assert!(gene.enabled);

        <dyn Mutation>::toggle_expression(&mut gene);
        assert!(!gene.enabled);
    }
}
