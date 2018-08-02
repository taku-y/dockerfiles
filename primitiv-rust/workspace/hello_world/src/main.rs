extern crate primitiv;

use std::collections::HashMap;

use primitiv::Graph;
//use primitiv::Optimizer;
use primitiv::Parameter;
use primitiv::Node;

use primitiv::devices as D;
use primitiv::initializers as I;
use primitiv::node_functions as F;
use primitiv::node_functions::random as R;
// use primitiv::optimizers as O;

use std::f32::consts::PI as PI;
//use std::ops::Mul;

enum ProcessMode {
    LOGP,
    SAMPLE,
}

#[derive(Debug)]
struct Model<'a> {
    pub samples: HashMap<&'a str, Node>,
    pub logp: Node
}

impl<'a> Model<'a> {
    fn new() -> Self {
        Model {
            samples: HashMap::new(),
            logp: F::constant([], 0.0),
        }
    }

    fn get_sample(&self, name: &str) -> &Node {
        match self.samples.get(name) {
            Some(sample) => sample,
            _ => panic!("Sample of RV '{}' not found", name)
        }
    }

    fn add_sample(&mut self, name: &'a str, sample: Node) {
        self.samples.insert(name, sample);
    }

    fn process<'b>(&mut self, name: &'a str, dist: &'b (Distribution + 'b),
                   mode: ProcessMode) -> &Node {
        match mode {
            ProcessMode::SAMPLE => {
                let sample = dist.sample();
                self.add_sample(name, sample);
                self.get_sample(name)
            }
            ProcessMode::LOGP => {
                let sample = match self.samples.get(name) {
                    Some(sample) => sample,
                    _ => panic!("Sample of RV '{}' not found", name),
                };
                let logp = &(self.logp) + dist.logp(sample);
                self.logp = logp;
                &sample
            }
        }
    }
}

trait Distribution {
    fn logp(&self, sample: &Node) -> Node;
    fn sample(&self) -> Node;
}

struct Normal {
    mean: Node,
    std: Node,
}

impl Normal {
    fn new(mean: Node, std: Node) -> Self {
        Normal {
            mean: mean,
            std: std
        }
    }
}

impl Distribution for Normal {
    fn logp(&self, sample: &Node) -> Node {
        let diff = sample - &(self.mean);
        let diff2 = &diff * &diff;
        let logp1 = -F::log(F::constant([], 2.0 * PI) * &(self.std));
        let logp2: Node = -0.5 * &diff2;

        logp1 + logp2
        //let logp4 = -F::log(F::constant([], 2.0 * PI) * &std) - F::constant([], 0.5) * diff2;
    }

    fn sample(&self) -> Node {
        let sample = R::normal(([2], 3), 0.0, 1.0);
        F::matmul(sample, &self.std) + &self.mean
    }
}

fn main() {
    // Device for primitiv
    let mut dev = D::Naive::new();
    D::set_default(&mut dev);

        // Initialize parameters
    let mut p_mean = Parameter::from_initializer([], &I::Constant::new(0.01));
    let mut p_lstd = Parameter::from_initializer([], &I::Constant::new(-0.1));

    // Graph for primitiv
    let mut g = Graph::new();
    Graph::set_default(&mut g);

    // Variational distribution
    let mut guide = | model: &mut Model, mode: ProcessMode | {
        //  Parameter nodes
        let mean = F::parameter(&mut p_mean);
        let lstd = F::parameter(&mut p_lstd);
        let std = F::exp(lstd);

        let _x = model.process(
            "w", &Normal::new(mean, std), mode
        );
    };

    // Inference loop
    //for i in 0..10 {
    {
        g.clear();

        // Generative model
        let mut model = Model::new();
        guide(&mut model, ProcessMode::SAMPLE);
        guide(&mut model, ProcessMode::LOGP);

        println!("logp = {:?}", model.logp.to_vector());
    }
}

//fn main() {
//    // Device for primitiv
//    let mut dev = D::Naive::new();
//    D::set_default(&mut dev);
//
//    // Initialize parameters
//    let mut p_mean = Parameter::from_initializer([], &I::Constant::new(0.01));
//    let mut p_lstd = Parameter::from_initializer([], &I::Constant::new(0.01));
//
//    // Graph for primitiv
//    let mut g = Graph::new();
//    Graph::set_default(&mut g);
//
//    // Start construction of graph
//    g.clear();
//
//    //  Nodes for parameters
//    let mean = F::parameter(&mut p_mean);
//    let lstd = F::parameter(&mut p_lstd);
//
//    // samples
//    let std = F::exp(lstd);
//    let samples = R::normal(([2], 3), 0.0, 1.0);
//    let samples = F::matmul(samples, &std) + &mean;
//    let result = samples.to_vector();
//
//    // logp
//    let diff = samples - mean;
//    let diff2 = (&diff * &diff);
//
//    let logp1 = -F::log(F::constant([], 2.0 * PI) * &std);
//    let logp2: Node = -0.5 * &diff2;
//    let logp3 = &logp1 + &logp2;
//    let logp4 = -F::log(F::constant([], 2.0 * PI) * &std) - F::constant([], 0.5) * diff2;
//
//    println!("{:?}", logp1.to_vector());
//    println!("{:?}", logp2.to_vector());
//    println!("{:?}", logp3.to_vector());
//    println!("{:?}", logp4.to_vector());
//}