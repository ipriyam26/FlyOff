use rand::{Rng};

pub struct  Network{
layers:Vec<Layer>
}

struct  Neuron{
    bias:f32,
    weights:Vec<f32>
}
struct  Layer{
    neurons:Vec<Neuron>,
}

impl  Network  {
    pub fn propogate(&self, inputs:Vec<f32>)->Vec<f32>{
        // we pass the input through each layer and return the output from the last layer back to the caller
        self.layers
        .iter()
        .fold(inputs,
             |inputs,layer|
             layer.propogate(inputs)
            )
    }
    pub fn random(layers: &Vec<LayerTopology>) -> Self {
        let layers = layers
        .windows(2)
        .map(|layers|{
Layer::random(layers[0].neurons, layers[1].neurons)
        }).collect();
        Self { layers: layers }
    }
    
}


impl  Neuron {
    fn propogate(&self,inputs:&Vec<f32>) -> f32{
let output = inputs
.iter()
.zip(&self.weights)
.map(|(input,weight)| input*weight)
.sum::<f32>();
(self.bias+output).max(0.0)
    }

    fn random(rng:&mut dyn rand::RngCore, input_neurons: usize) -> Neuron {

        
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_neurons)
        .map(|_| rng.gen_range(-1.0..=1.0))  
        .collect();
        Self { bias: bias, weights: weights }
    }
}



#[cfg(test)]
mod tests{
    use super::*;
    mod random{
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::{ ChaCha8Rng};

        use super::*;
        
        #[test]
        fn test(){
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng,4);
            assert_relative_eq!(neuron.bias,-0.6255188);
            assert_relative_eq!(neuron.weights.as_slice(),[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref())
        }
    }
}




pub struct  LayerTopology{
    neurons:usize
}

impl  Layer {
    
    fn propogate(&self,inputs:Vec<f32>) -> Vec<f32>{
       self.neurons
       .iter()
       .map(|neuron| neuron.propogate(&inputs))
       .collect()
    }

    pub fn random(
        input_neurons: usize,
        output_neurons: usize,
    ) -> Self {
        let  neurons = (0..output_neurons).map(|_| Neuron::random(&mut rand::thread_rng(),input_neurons)).collect();
        Self { neurons: neurons }
    }

}

