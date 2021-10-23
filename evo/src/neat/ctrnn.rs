use rulinalg::matrix::{BaseMatrix, BaseMatrixMut, Matrix};

#[cfg(feature = "ctrnn_telemetry")]
use rusty_dashed;

#[cfg(feature = "ctrnn_telemetry")]
use serde_json;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct CtrnnNeuralNetwork<'a> {
    pub y: &'a [f64],     //current state of neuron(j)
    pub tau: &'a [f64], //τ - time constant ( t > 0 ). The neuron's speed of response to an external sensory signal. Membrane resistance time.
    pub wji: &'a [f64], //w - weights of the connection from neuron(j) to neuron(i)
    pub theta: &'a [f64], //θ - bias of the neuron(j)
    pub i: &'a [f64],   //I - external input to neuron(i)
}

#[allow(missing_docs)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Ctrnn {}

impl Ctrnn {
    /// Activate the NN
    pub fn activate_nn(&self, time: f64, step_size: f64, nn: &CtrnnNeuralNetwork) -> Vec<f64> {
        let steps = (time / step_size) as usize;
        let mut y = Ctrnn::vector_to_column_matrix(nn.y);
        let theta = Ctrnn::vector_to_column_matrix(nn.theta);
        let wji = Ctrnn::vector_to_matrix(nn.wji);
        let i = Ctrnn::vector_to_column_matrix(nn.i);
        let tau = Ctrnn::vector_to_column_matrix(nn.tau);

        #[cfg(feature = "ctrnn_telemetry")]
        Ctrnn::telemetry(&y);

        for _ in 0..steps {
            let current_weights = (&y + &theta).apply(&f64::tanh);
            y = &y
                + ((&wji * current_weights) - &y + &i)
                    .elediv(&tau)
                    .apply(&|j_value| step_size * j_value);
            #[cfg(feature = "ctrnn_telemetry")]
            Ctrnn::telemetry(&y);
        }
        y.into_vec()
    }

    /// Calculates sigmoid of a number
    pub fn sigmoid(x: f64) -> f64 {
        1f64 / (1f64 + (-x).exp())
    }

    fn vector_to_column_matrix(vector: &[f64]) -> Matrix<f64> {
        Matrix::new(vector.len(), 1, vector)
    }

    fn vector_to_matrix(vector: &[f64]) -> Matrix<f64> {
        let width = (vector.len() as f64).sqrt() as usize;
        Matrix::new(width, width, vector)
    }

    #[cfg(feature = "ctrnn_telemetry")]
    fn telemetry(y: &Matrix<f64>) {
        let y2 = y.clone();
        telemetry!(
            "ctrnn1",
            1.0,
            serde_json::to_string(&y2.into_vec()).unwrap()
        );
    }
}

