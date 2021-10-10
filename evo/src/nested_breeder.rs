use crate::breeder::*;
use crate::neat::NeatBreeder;
use std::collections::HashMap;

/// Nested Breeder {{{1
enum NestedBreeder {
    Vec(VecBreeder),
    Float(FloatBreeder),
    Network(NeatBreeder),

    List(Vec<NestedBreeder>),
    Map(HashMap<String, NestedBreeder>),
}

impl From<FloatBreeder> for NestedBreeder {
    fn from(b: FloatBreeder) -> Self {
        NestedBreeder::Float(b)
    }
}
impl From<VecBreeder> for NestedBreeder {
    fn from(b: VecBreeder) -> Self {
        NestedBreeder::Vec(b)
    }
}
impl From<NeatBreeder> for NestedBreeder {
    fn from(b: NeatBreeder) -> Self {
        NestedBreeder::Network(b)
    }
}
impl From<Vec<NestedBreeder>> for NestedBreeder {
    fn from(b: Vec<NestedBreeder>) -> Self {
        NestedBreeder::List(b)
    }
}
impl From<HashMap<String, NestedBreeder>> for NestedBreeder {
    fn from(b: HashMap<String, NestedBreeder>) -> Self {
        NestedBreeder::Map(b)
    }
}
impl From<HashMap<&str, NestedBreeder>> for NestedBreeder {
    fn from(b: HashMap<&str, NestedBreeder>) -> Self {
        NestedBreeder::Map(b.into_iter().map(|(k, b)| (k.to_string(), b)).collect())
    }
}

#[derive(Clone, Debug)]
enum NestedBreederGenome {
    Vec(<VecBreeder as Breeder>::Genome),
    Float(<FloatBreeder as Breeder>::Genome),
    Network(<NeatBreeder as Breeder>::Genome),

    List(Vec<NestedBreederGenome>),
    Map(HashMap<String, NestedBreederGenome>),
}

impl NestedBreederGenome {
    fn unwrap_vec(&self) -> &<VecBreeder as Breeder>::Genome {
        match self {
            NestedBreederGenome::Vec(x) => x,
            _ => panic!("Failed to Unwrap"),
        }
    }
    fn unwrap_float(&self) -> &<FloatBreeder as Breeder>::Genome {
        match self {
            NestedBreederGenome::Float(x) => x,
            _ => panic!("Failed to Unwrap"),
        }
    }
    fn unwrap_network(&self) -> &<NeatBreeder as Breeder>::Genome {
        match self {
            NestedBreederGenome::Network(x) => x,
            _ => panic!("Failed to Unwrap"),
        }
    }
    fn unwrap_list(&self) -> &Vec<NestedBreederGenome> {
        match self {
            NestedBreederGenome::List(x) => x,
            _ => panic!("Failed to Unwrap"),
        }
    }
    fn unwrap_map(&self) -> &HashMap<String, NestedBreederGenome> {
        match self {
            NestedBreederGenome::Map(x) => x,
            _ => panic!("Failed to Unwrap"),
        }
    }
}

impl Breeder for NestedBreeder {
    type Genome = NestedBreederGenome;

    fn mutate(&self, gene: &Self::Genome) -> Self::Genome {
        match self {
            NestedBreeder::Vec(b) => NestedBreederGenome::Vec(b.mutate(gene.unwrap_vec())),
            NestedBreeder::Float(b) => NestedBreederGenome::Float(b.mutate(gene.unwrap_float())),
            NestedBreeder::Network(b) => {
                NestedBreederGenome::Network(b.mutate(gene.unwrap_network()))
            }
            NestedBreeder::List(l) => NestedBreederGenome::List(
                l.iter()
                    .zip(gene.unwrap_list())
                    .map(|(b, g)| b.mutate(g))
                    .collect(),
            ),
            NestedBreeder::Map(l) => {
                let g = gene.unwrap_map();
                NestedBreederGenome::Map(
                    l.iter()
                        .map(|(k, b)| (k.clone(), b.mutate(g.get(k).unwrap())))
                        .collect(),
                )
            }
        }
    }

    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome {
        match self {
            NestedBreeder::Vec(b) => {
                let g1 = gene1.unwrap_vec();
                let g2 = gene2.unwrap_vec();

                NestedBreederGenome::Vec(b.breed(g1, g2))
            }
            NestedBreeder::Float(b) => {
                let g1 = gene1.unwrap_float();
                let g2 = gene2.unwrap_float();

                NestedBreederGenome::Float(b.breed(g1, g2))
            }
            NestedBreeder::Network(b) => {
                let g1 = gene1.unwrap_network();
                let g2 = gene2.unwrap_network();

                NestedBreederGenome::Network(b.breed(g1, g2))
            }
            NestedBreeder::List(l) => {
                let g1 = gene1.unwrap_list();
                let g2 = gene2.unwrap_list();

                NestedBreederGenome::List(
                    l.iter()
                        .zip(g1)
                        .zip(g2)
                        .map(|((b, g1), g2)| b.breed(g1, g2))
                        .collect(),
                )
            }
            NestedBreeder::Map(l) => {
                let g1 = gene1.unwrap_map();
                let g2 = gene2.unwrap_map();

                NestedBreederGenome::Map(
                    l.iter()
                        .map(|(k, b)| (k.clone(), b.breed(g1.get(k).unwrap(), g2.get(k).unwrap())))
                        .collect(),
                )
            }
        }
    }

    fn random(&self) -> Self::Genome {
        match self {
            NestedBreeder::Vec(b) => NestedBreederGenome::Vec(b.random()),
            NestedBreeder::Float(b) => NestedBreederGenome::Float(b.random()),
            NestedBreeder::Network(b) => NestedBreederGenome::Network(b.random()),
            NestedBreeder::List(l) => {
                NestedBreederGenome::List(l.iter().map(|b| b.random()).collect())
            }
            NestedBreeder::Map(m) => {
                NestedBreederGenome::Map(m.iter().map(|(k, v)| (k.clone(), v.random())).collect())
            }
        }
    }

    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
        match self {
            NestedBreeder::Vec(b) => b.is_same(gene1.unwrap_vec(), gene2.unwrap_vec()),
            NestedBreeder::Float(b) => b.is_same(gene1.unwrap_float(), gene2.unwrap_float()),
            NestedBreeder::Network(b) => b.is_same(gene1.unwrap_network(), gene2.unwrap_network()),
            NestedBreeder::List(l) => {
                let g1 = gene1.unwrap_list();
                let g2 = gene2.unwrap_list();

                l.iter()
                    .zip(g1)
                    .zip(g2)
                    .fold(true, |v, ((b, g1), g2)| v && b.is_same(g1, g2))
            }
            NestedBreeder::Map(m) => {
                let g1 = gene1.unwrap_map();
                let g2 = gene2.unwrap_map();

                m.iter().fold(true, |v, (k, b)| {
                    v && b.is_same(g1.get(k).unwrap(), g2.get(k).unwrap())
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::breeder::*;
    use maplit::hashmap;

    // #[test]
    fn test_nested() {
        let b: NestedBreeder = vec![
            VecBreeder::default().into(),
            VecBreeder::default().into(),
            FloatBreeder {
                min: 0.0,
                max: 1.0,
                delta: 0.1,
            }
            .into(),
            hashmap! {
                "foo" => VecBreeder::default().into()
            }
            .into(),
        ]
        .into();

        let g = b.random();
        println!("{:?}", g);
    }
}
