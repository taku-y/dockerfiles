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

pub enum ProcessMode {
    LOGP,
    SAMPLE,
}

#[derive(Debug)]
pub struct Model<'a> {
    pub samples: HashMap<&'a str, Node>,
    pub logp: Node
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
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

    pub fn process<'b>(&mut self, name: &'a str, dist: &'b (Distribution + 'b),
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

pub trait Distribution {
    fn logp(&self, sample: &Node) -> Node;
    fn sample(&self) -> Node;
}

pub struct Normal {
    mean: Node,
    std: Node,
}

impl Normal {
    pub fn new(mean: Node, std: Node) -> Self {
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
