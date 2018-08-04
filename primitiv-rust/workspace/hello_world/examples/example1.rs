extern crate rand;
extern crate primitiv;
extern crate hello_world;

use std::collections::HashMap;

use rand::distributions::Distribution;
use rand::distributions::normal::Normal as NormalInRand;

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

use hello_world::{Model, ProcessMode, Normal};

fn main() {
    // Create sample data
    let n = NormalInRand::new(-3.5, 1.0);
    let mut rng = rand::thread_rng();
    let data = (0..100).map(|_| n.sample(&mut rng) as f32).collect::<Vec<_>>();

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
    for _i in 0..10
    {
        g.clear();

        // Generative model
        let mut model = Model::new();
        guide(&mut model, ProcessMode::SAMPLE);
        guide(&mut model, ProcessMode::LOGP);

        println!("logp = {:?}", model.logp.to_vector());
    }
}
