use std::iter::once;

use rand::{Rng, RngCore};

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Clone, Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

#[derive(Clone, Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Network {
    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }
    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }

    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // we pass the input through each layer and return the output from the last layer back to the caller
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers: layers }
    }
}

impl Neuron {
    // pub(crate) fn new()
    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }

    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    fn propogate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    }

    fn random(rng: &mut dyn rand::RngCore, input_neurons: usize) -> Neuron {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_neurons)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
        Self {
            bias: bias,
            weights: weights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual: Vec<f32> = network.weights().collect();
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
        }
    }
    mod random {
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);
            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(
                neuron.weights.as_slice(),
                [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
            )
        }
    }

    mod propogate {
        use approx::assert_relative_eq;

        use super::*;

        #[test]
        fn test() {
            let neuron = Neuron {
                bias: 0.3,
                weights: vec![-0.5, 0.8],
            };

            // a simple check for relu to work

            assert_relative_eq!(neuron.propogate(&vec![0.4, -1.0]), 0.0);

            assert_relative_eq!(
                neuron.propogate(&vec![0.5, 1.0]),
                (-0.5 * 0.5) + (0.8 * 1.0) + 0.3,
            );
        }
    }
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self { neurons }
    }
    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }

    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }

    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rand::thread_rng(), input_neurons))
            .collect();
        Self { neurons: neurons }
    }
}
