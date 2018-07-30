extern crate primitiv;

use std::collections::HashMap;

use primitiv::Graph;
use primitiv::Optimizer;
use primitiv::Parameter;
use primitiv::Node;

use primitiv::devices as D;
use primitiv::initializers as I;
use primitiv::node_functions as F;
use primitiv::node_functions::random as R;
use primitiv::optimizers as O;

use std::f32::consts::PI as PI;
use std::ops::Mul;

#[derive(Debug)]
struct Model {
    samples: HashMap<String, Node>
}

impl Model {
    fn new() -> Self {
        Model {
            samples: HashMap::new(),
        }
    }

    fn draw_samples(&mut self, d: &mut Distribution) -> &Node {
        let samples = d.draw_samples();
        self.samples.insert(d.name().clone().to_string(), samples);
        &(self.samples.get(d.name()).unwrap())
    }
}

struct Normal {}

impl Normal {
    fn new(name: &'a str, mean: Node, std: Node) -> Self {
        Normal {
            name: name,
            mean: mean,
            std: std
        }
    }
}

impl Distribution {
    fn proc(&self, m: &mut Model) -> & Node {
        match m.mode {
            Mode::sample => {}
            Mode::logp => {}
        }
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

    // Generative model
    let mut m = Model::new();

    // Variational distribution
    let draw_samples = | m: &mut Model | {
        //  Parameter nodes
        let mean = F::parameter(&mut p_mean);
        let lstd = F::parameter(&mut p_lstd);
        let std = F::exp(lstd);

        let x = Normal::draw("w", mean, F::exp(lstd)).proc(m);
    };

    // Inference loop
    //for i in 0..10 {
    {
        g.clear();

        draw_samples(&mut m);

        println!("{:?}", m);
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