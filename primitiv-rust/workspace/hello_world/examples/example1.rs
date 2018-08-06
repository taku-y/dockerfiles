extern crate rand;
extern crate primitiv;
extern crate hello_world;

use rand::distributions::Distribution;
use rand::distributions::normal::Normal as NormalInRand;

use primitiv::Graph;
use primitiv::Optimizer;
use primitiv::Parameter;

use primitiv::devices as D;
use primitiv::initializers as I;
use primitiv::node_functions as F;
use primitiv::optimizers as O;

use hello_world::{RandomVarManager, ProcessMode, Normal};

fn main() {
    // Create sample data
    let n_samples = 100;
    let n = NormalInRand::new(-3.5, 1.0);
    let mut rng = rand::thread_rng();
    let data = (0..n_samples).map(|_| n.sample(&mut rng) as f32).collect::<Vec<_>>();

    // Device for primitiv
    let mut dev = D::Naive::new();
    D::set_default(&mut dev);

    // Initialize parameters
    let mut p_mean = Parameter::from_initializer([], &I::Constant::new(0.01));
    let mut p_lstd = Parameter::from_initializer([], &I::Constant::new(-0.1));

    // Optimizer
    let mut optimizer = O::SGD::new(0.02);
    optimizer.add_parameter(&mut p_mean);
    optimizer.add_parameter(&mut p_lstd);

    // Graph for primitiv
    let mut g = Graph::new();
    Graph::set_default(&mut g);

    {
        // Normal distribution
        let mut model = |rvm: &mut RandomVarManager, mode: ProcessMode| {
            //  Parameter nodes
            let mean = F::parameter(&mut p_mean);
            let lstd = F::parameter(&mut p_lstd);
            let std = F::exp(lstd);

            let _x = rvm.process("x", &Normal::new(mean, std), mode);
        };

        // Inference loop
        for _i in 0..500
        {
            g.clear();

            // Generative model
            let mut rvm = RandomVarManager::new();
            rvm.add_sample("x", F::input(([], n_samples), &data));
            // model(&mut model, ProcessMode::SAMPLE);
            model(&mut rvm, ProcessMode::LOGP);

            // Objective function
            let nlogp = -F::batch::mean(&(rvm.logp));

            println!("nlogp = {:?}", nlogp.to_float());

            optimizer.reset_gradients();
            nlogp.backward();
            optimizer.update();
        }
    } // Brrowing p_mean and p_lstd ends here

    println!("(mean, lstd) = ({}, {})", p_mean.value().to_float(),
             p_lstd.value().to_float().exp());
}
