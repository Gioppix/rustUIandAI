pub mod network {
    use rand::Rng;

    #[derive(Copy, Clone)]
    pub enum ActivationFunction {
        ReLU,
        TANH,
    }

    pub struct LayerTopology {
        pub neurons: usize,
        pub activation_function: ActivationFunction,
    }

    #[derive(Clone)]
    pub struct Network {
        layers: Vec<Layer>,
    }

    impl Network {
        pub fn mutate(&self, other: &Self, mutation_rate: f32, mutation: f32) -> Self {
            let mut new_layers = Vec::new();
            for (layer1, layer2) in self.layers.iter().zip(other.layers.iter()) {
                new_layers.push(layer1.mutate(layer2, mutation_rate, mutation));
            }
            Network {
                layers: new_layers
            }
        }
        pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
            for layer in &self.layers {
                inputs = layer.propagate(inputs);
            }

            inputs
        }
        pub fn random(layers: &[LayerTopology]) -> Self {
            assert!(layers.len() > 1);
            let layers = layers
                .windows(2)
                .map(|layers| {
                    Layer::random(layers[0].neurons, layers[1].neurons, layers[1].activation_function)
                })
                .collect();

            Self { layers }
        }
    }

    #[derive(Clone)]
    struct Layer {
        neurons: Vec<Neuron>,
    }

    impl Layer {
        pub fn mutate(&self, other: &Self, mutation_rate: f32, mutation: f32) -> Self {
            let mut new_neurons = Vec::new();
            for (neu1, neu2) in self.neurons.iter().zip(other.neurons.iter()) {
                new_neurons.push(neu1.mutate(neu2, mutation_rate, mutation));
            }
            Layer {
                neurons: new_neurons,
            }
        }
        fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
            self.neurons
                .iter()
                .map(|neuron| neuron.propagate(&inputs))
                .collect()
        }
        pub fn random(input_size: usize, output_size: usize, activation_function: ActivationFunction) -> Self {
            let neurons = (0..output_size)
                .map(|_| Neuron::random(input_size, activation_function))
                .collect();

            Self { neurons }
        }
    }

    #[derive(Clone)]
    struct Neuron {
        bias: f32,
        weights: Vec<f32>,
        activation_function: ActivationFunction,
    }

    fn mutate_value(val1: f32, val2: f32, mutation_rate: f32, mutation: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let val = if rng.gen_bool(0.5) { val1 } else { val2 };
        if rng.gen_bool(mutation_rate as f64) {
            val + rng.gen_range(-mutation..=mutation)
        } else {
            val
        }
    }

    impl Neuron {
        fn mutate(&self, other: &Self, mutation_rate: f32, mutation: f32) -> Self {
            //let mut rng = rand::thread_rng();
            //rng.gen_range(-var..=var)
            let new_b = mutate_value(self.bias, other.bias, mutation_rate, mutation);
            let new_w = self.weights.iter().zip(other.weights.iter()).map(|(w1, w2)| mutate_value(*w1, *w2, mutation_rate, mutation)).collect();

            Self {
                bias: new_b,
                weights: new_w,
                activation_function: self.activation_function,
            }
        }
        fn propagate(&self, inputs: &[f32]) -> f32 {
            if self.weights.len() != inputs.len() {
                println!("{}, {}", self.weights.len(), inputs.len());
                panic!("input sbagliati");
            }
            let output = inputs
                .iter()
                .zip(&self.weights)
                .map(|(input, weight)| input * weight)
                .sum::<f32>();

            match self.activation_function {
                ActivationFunction::ReLU => {
                    (self.bias + output).max(0.0)
                }
                ActivationFunction::TANH => {
                    (self.bias + output).tanh()
                }
            }
        }
        pub fn random(input_size: usize, activation_function: ActivationFunction) -> Self {
            let mut rng = rand::thread_rng();

            let bias = rng.gen_range(-1.0..=1.0);

            let weights = (0..input_size)
                .map(|_| rng.gen_range(-1.0..=1.0))
                .collect();

            Self { bias, weights, activation_function }
        }
    }
}