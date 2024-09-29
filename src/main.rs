use rand::Rng;
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
    let test = Weights::create(2, 2);
    let matrix = Weights {
        matrix: vec![
            vec![1., 2., 3.],
            vec![4., 5., 6.],
            vec![7., 8., 9.]
        ],
    };
    let vector = vec![
        10.,
        11.,
        12.
    ];
    println!("{:?}",matrix.vectormultiply(vector));
}
