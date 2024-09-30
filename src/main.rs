use img::{images_from_xz, Img};
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

mod img;

#[derive(Clone)]
struct NeuralNetwork {
    layers: Vec<Layer>,
    score: f64,
}

impl NeuralNetwork {
    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.layers.iter_mut().for_each(|layer| {
            layer.weights.iter_mut().flatten().for_each(|value| {
                *value += rng.gen_range(-0.01..0.01);
            });
            layer.bias.iter_mut().for_each(|value| {
                *value += rng.gen_range(-0.01..0.01);
            })
        })
    }

    fn scoring(&mut self, result: &[f64], wanted: usize) {
        self.score = (result[wanted] * 2.) - result.iter().sum::<f64>();
    }

    fn create(
        input_layer_size: usize,
        hidden_layer_amount: usize,
        hidden_layer_size: usize,
        output_layer_size: usize,
    ) -> NeuralNetwork {
        let mut connections = vec![];
        //creates input layer weights and adds them to weights
        connections.push(Layer::create(input_layer_size, hidden_layer_size));
        //creates hidden layer weights and adds them to weights
        (0..hidden_layer_amount)
            .for_each(|_| connections.push(Layer::create(hidden_layer_size, hidden_layer_size)));
        //creats output layer weights and adds them to weights
        connections.push(Layer::create(hidden_layer_size, output_layer_size));
        NeuralNetwork {
            layers: connections,
            score: 0.,
        }
    }

    fn evaluate(&self, input_vector: Vec<f64>) -> Vec<f64> {
        self.layers.iter().fold(input_vector, |vector, layer| {
            layer
                .vectormultiply(&vector)
                .iter()
                .zip(&layer.bias)
                .map(|(value, bias)| value + bias)
                .map(sigmoid)
                .collect()
        })
    }
}

#[derive(Clone)]
struct Layer {
    weights: Vec<Vec<f64>>,
    bias: Vec<f64>,
}
impl Layer {
    fn create(input: usize, output: usize) -> Layer {
        let mut weights: Vec<Vec<f64>> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..output {
            let neuron_connections: Vec<f64> =
                (0..input).map(|_| rng.gen_range(-2.0..2.0)).collect();
            weights.push(neuron_connections);
        }
        let bias = (0..output).map(|_| rng.gen_range(-2.0..2.0)).collect();
        Layer { weights, bias }
    }

    fn vectormultiply(&self, vector: &[f64]) -> Vec<f64> {
        let mut return_vector: Vec<f64> = vec![];
        for row in &self.weights {
            let new_entry: f64 = row
                .iter()
                .zip(vector)
                .map(|(value, vector)| f64::max(0., value * vector))
                .sum();
            return_vector.push(new_entry);
        }
        return_vector
    }
}

struct NetworkPool {
    pub nets: Vec<NeuralNetwork>,
}
impl NetworkPool {
    pub fn create(network: NeuralNetwork, network_amount: usize) -> Self {
        Self {
            nets: vec![network; network_amount],
        }
    }
    pub fn eval(&mut self, images: &[Img]) {
        self.nets.par_iter_mut().for_each(|neural_network| {
            for image in images {
                let result = neural_network.evaluate(image.data.clone());
                neural_network.scoring(&result, image.number as usize);
            }
        });
    }
    pub fn sort_by_score(&mut self) {
        self.nets
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }
    pub fn change(&mut self) {
        self.nets.iter_mut().for_each(|net| net.randomize());
    }
    pub fn train(&mut self, images: &[Img]) {
        self.eval(images);
        self.sort_by_score();

        let best = self.nets[0].clone();
        self.nets.fill(best.clone());

        self.change();

        self.eval(images);
        self.sort_by_score();

        if self.nets[0].score < best.score {
            self.nets[0] = best;
        }
    }
}

pub fn sigmoid(input: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-input))
}

fn main() {
    let images = images_from_xz("./data/train.xz");

    let mut pool = NetworkPool::create(NeuralNetwork::create(images[0].data.len(), 5, 20, 10), 50);

    loop {
        pool.train(&images[0..1]);
        println!("score: {}", pool.nets[0].score);
    }
}
