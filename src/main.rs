use rand::Rng;

mod img;

struct NeuralNetwork {
    weights: Vec<Weights>,
}

impl NeuralNetwork {
    fn create(
        input_layer_size: u64,
        hidden_layer_amount: u64,
        hidden_layer_size: u64,
        output_layer_size: u64,
    ) -> NeuralNetwork {
        let mut weights = vec![];
        //creates input layer weights and adds them to weights
        weights.push(Weights::create(input_layer_size, hidden_layer_size));
        //creates hidden layer weights and adds them to weights
        (0..hidden_layer_amount)
            .for_each(|_| weights.push(Weights::create(hidden_layer_size, hidden_layer_size)));
        //creats output layer weights and adds them to weights
        weights.push(Weights::create(hidden_layer_size, output_layer_size));
        NeuralNetwork { weights }
    }
    fn evaluate(&self, input_vector: Vec<f64>) -> Vec<f64> {
        self.weights
            .iter()
            .fold(input_vector, |result, weight| weight.vectormultiply(result))
    }
}

struct Weights {
    matrix: Vec<Vec<f64>>,
}
impl Weights {
    fn create(input: u64, output: u64) -> Weights {
        let mut matrix: Vec<Vec<f64>> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..output {
            let neuron_connections: Vec<f64> =
                (0..input).map(|_| rng.gen_range(-2.0..2.0)).collect();
            matrix.push(neuron_connections);
        }
        Weights { matrix }
    }

    fn vectormultiply(&self, vector: Vec<f64>) -> Vec<f64> {
        let mut return_vector: Vec<f64> = vec![];
        for row in &self.matrix {
            let new_entry: f64 = row
                .iter()
                .enumerate()
                .map(|(index, &value)| value * vector[index])
                .sum();
            return_vector.push(new_entry);
        }
        return_vector
    }
}
fn main() {
    let neural_network = NeuralNetwork::create(2, 2, 2, 2);
    let result = neural_network.evaluate(vec![10., 10.]);
}
