//use crate::breeder::Breeder;
//use crate::utils::{random, random_d};

//type Signal<const N: usize> = [f64; N];

//#[derive(Debug, Clone)]
//struct Cell<const N: usize> {
//    weights: [[f64; N]; N],
//    biases: [f64; N],
//}

//impl<const N: usize> Cell<N> {
//    fn mutate(&self, rate: f64, amt: f64) -> Cell<N> {
//        let mut new_cell = self.clone();
//        for x in 0..N {
//            for y in 0..N {
//                if random() < rate {
//                    new_cell.weights[x][y] += random_d(amt);
//                }
//            }

//            if random() < rate {
//                new_cell.biases[x] += random_d(amt);
//            }
//        }
//        new_cell
//    }

//    fn breed(&self, other: &Self, rate: f64) -> Cell<N> {
//        let mut new_cell = self.clone();
//        let mut flip = false;

//        for x in 0..N {
//            for y in 0..N {
//                if random() < rate {
//                    flip = !flip;
//                }

//                if flip {
//                    new_cell.weights[x][y] = other.weights[x][y]
//                }
//            }

//        }

//        for x in 0..N {
//            if random() < rate {
//                flip = !flip;
//            }

//            if flip {
//                new_cell.biases[x] = other.biases[x];
//            }
//        }

//        new_cell
//    }

//    fn random() -> Cell<N> {
//        let mut weights = [[0.0; N]; N];
//        let mut biases = [0.0; N];

//        for x in 0..N {
//            biases[x] = random_d(2.0);

//            for y in 0..N {
//                weights[x][y] = random_d(2.0);
//            }
//        }

//        Self { weights, biases }
//    }
//}


//#[derive(Debug)]
//struct CellNetwork<const N: usize> {
//    pub state: Signal<N>,
//    cells: Vec<Cell<N>>,
//}

//struct CellNetworkBreeder<const N: usize> {
//    initial_cells: i32,
//    cell_mutate_rate: f64,
//    cell_mutate_amt: f64,
//    cell_flip_rate: f64,
//}

//impl<const N: usize> Breeder for CellNetworkBreeder<N> {
//    type Genome = CellNetwork<N>;

//    fn mutate(&self, gene: &Self::Genome) -> Self::Genome {
//        CellNetwork {
//            state: [0.0; N],
//            cells: gene
//                .cells
//                .iter()
//                .map(|c| c.mutate(self.cell_mutate_rate, self.cell_mutate_amt))
//                .collect(),
//        }
//    }

//    fn breed(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> Self::Genome {
//        CellNetwork {
//            state: [0.0; N],
//            cells: gene1
//                .cells
//                .iter()
//                .zip(gene2.cells.iter())
//                .map(|(c1, c2)| c1.breed(&c2, self.cell_flip_rate))
//                .collect(),
//        }
//    }

//    fn random(&self) -> Self::Genome {
//        CellNetwork {
//            state: [0.0; N],
//            cells: (0..self.initial_cells)
//                .map(|_| Cell::<N>::random())
//                .collect(),
//        }
//    }

//    fn is_same(&self, gene1: &Self::Genome, gene2: &Self::Genome) -> bool {
//        true
//    }
//}

//impl<const N: usize> Clone for CellNetwork<N> {
//    fn clone(&self) -> Self {
//        Self {
//            state: [0.0; N],
//            cells: self.cells.clone(),
//        }
//    }
//}

///// Inputs [1, 0, 0], [0, 1, 0]
///// Outputs [0, 0, 1]
/////
///// 0 1
///// 0 0
///// 0 1
/////
//impl<const N: usize> Cell<N> {
//    fn process(&self, input: &Signal<N>) -> Signal<N> {
//        let mut output: Signal<N> = self.biases.clone();
//        for x in 0..N {
//            for y in 0..N {
//                output[x] += self.weights[x][y] * input[y];
//            }
//        }

//        output.iter_mut().for_each(|x| *x = x.tanh());
//        output
//    }
//}

//impl<const N: usize> CellNetwork<N> {
//    fn update(&mut self, dt: f64, input: &Signal<N>) {
//        for i in 0..N {
//            self.state[i] += (input[i] - self.state[i]) * dt;
//        }
//        for cell in &self.cells {
//            let out = cell.process(&self.state);
//            for x in 0..N {
//                self.state[x] += dt * out[x];
//            }
//        }
//    }

//    fn normalize(&mut self) {
//        self.state.iter_mut().for_each(|x| *x = x.clamp(-1.0, 1.0));
//    }

//}

//#[cfg(test)]
//mod test {
//    use super::*;
//    use crate::pool::*;
//    use crate::breeder::*;
//    use crate::utils::*;

//    macro_rules! assert_delta {
//        ($x:expr, $y:expr, $d:expr) => {
//            assert!(($x - $y).abs() < $d);
//        };
//    }

//    #[test]
//    fn test_breed_cell_network() {
//        let b = CellNetworkBreeder::<5> {
//            initial_cells: 1,
//            cell_mutate_rate: 0.01,
//            cell_mutate_amt: 2.0,
//            cell_flip_rate: 0.01,
//        };

//        let mut pool = Pool::new(100, b);
//        pool.ratios = Ratios {
//            top: 0.3,
//            mutate: 0.4,
//            cross: 0.0,
//            random: 0.3,
//        };

//        for i in 0..1000 {
//            for i in 0..100 {
//                let mut c = pool.next().unwrap();
//                let mut fitness = 0.0;
//                let dt = 0.5;

//                let input = [1.0, -1.0, 0.0, 0.0, 0.0];
//                for _ in 0..100 {
//                    c.update(dt, &input);
//                }
//                c.normalize();
//                let f1 = c.state[2];
//                fitness += (f1 - 1.0).abs();

//                let input = [-1.0, 1.0, 0.0, 0.0, 0.0];
//                for _ in 0..100 {
//                    c.update(dt, &input);
//                }
//                c.normalize();
//                let f2 = c.state[2];
//                fitness += (f2 - 1.0).abs();

//                let input = [-1.0, -1.0, 0.0, 0.0, 0.0];
//                for _ in 0..100 {
//                    c.update(dt, &input);
//                }
//                c.normalize();
//                let f3 = c.state[2];
//                fitness += (f3 + 1.0).abs();

//                let input = [ 1.0,  1.0, 0.0, 0.0, 0.0];
//                for _ in 0..100 {
//                    c.update(dt, &input);
//                }
//                c.normalize();
//                let f4 = c.state[2];
//                fitness += (f4 + 1.0).abs();

//                if pool.reported_count() == 99 {
//                    println!("{}, {}, {}, {}", f1, f2, f3, f4);
//                }

//                pool.report(fitness, c);
//            }
//        }
//    }

//    #[test]
//    fn test_cell_identity() {
//        let c = Cell {
//            weights: [
//                [1.0, 0.0, 0.0],
//                [0.0, 1.0, 0.0],
//                [0.0, 0.0, 1.0],
//            ],
//            biases: [0.0, 0.0, 0.0],
//        };

//        let r = c.process(&[0.1, 0.2, 0.3]);
//        assert_delta!(r[0], 0.1, 0.01);
//        assert_delta!(r[1], 0.2, 0.01);
//        assert_delta!(r[2], 0.3, 0.01);

//        println!("{:?}", r);
//    }

//    #[test]
//    fn test_cell_xor() {
//        let c1 = Cell {
//            weights: [
//                [0.0, 0.0, 0.0],
//                [0.0, 0.0, 0.0],
//                [0.6, 0.6, 0.0],
//            ],
//            biases: [0.0, 0.0, -1.0],
//        };

//        let c2 = Cell {
//            weights: [
//                [0.0, 0.0, 0.0],
//                [0.0, 0.0, 0.0],
//                [1.1, 1.1, 0.0],
//            ],
//            biases: [0.0, 0.0, 1.0],
//        };

//        let c3 = Cell {
//            weights: [
//                [1.0, 1.0, 1.0],
//                [0.0, 0.0, 0.0],
//                [0.0, 0.0, 0.0],
//            ],
//            biases: [0.0, 0.0, 0.0],
//        };
//    }

//    ///
//    /// x   o
//    ///
//    /// o   x
//    ///
//    #[test]
//    fn test_manual_xor() {
//        let c = CellNetwork {
//            state: [0.0; 3],
//            cells: vec![
//                Cell {
//                    weights: [
//                        [1.0, 0.0, 0.0],
//                        [0.0, 0.0, 0.0],
//                        [0.0, 0.0, 0.0],
//                    ],
//                    biases: [0.0, 0.0, 0.0],
//                },
//                Cell {
//                    weights: [
//                        [0.0, 0.0, 0.0],
//                        [0.0, 0.0, 0.0],
//                        [0.0, 0.0, 0.0],
//                    ],
//                    biases: [0.0, 0.0, 0.0],
//                }
//            ],
//        };
//    }
//}
