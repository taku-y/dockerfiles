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
    // Create sample data from a linear regression model
    let n_samples = 100;
    let true_w = 2.5 as f32;
    let true_b = -1.0 as f32;
    let dist_x = NormalInRand::new(0.0, 10.0);
    let dist_e = NormalInRand::new(0.0, 0.1);
    let mut rng = rand::thread_rng();
    let xs = (0..n_samples).map(|_| dist_x.sample(&mut rng) as f32).collect::<Vec<_>>();
    let es = (0..n_samples).map(|_| dist_e.sample(&mut rng) as f32).collect::<Vec<_>>();
    let ys = xs.iter().zip(es.iter()).map(
        |(x, e)| true_w * x + true_b + e).collect::<Vec<_>>();

    // Device for primitiv
    let mut dev = D::Naive::new();
    D::set_default(&mut dev);

    // Initialize parameters
    let mut p_w = Parameter::from_initializer([], &I::Constant::new(0.01));
    let mut p_b = Parameter::from_initializer([], &I::Constant::new(-0.1));

    // Optimizer
    let mut optimizer = O::SGD::new(0.0001);
    optimizer.add_parameter(&mut p_w);
    optimizer.add_parameter(&mut p_b);

    // Graph for primitiv
    let mut g = Graph::new();
    Graph::set_default(&mut g);

    {
        // Normal distribution
        let mut model =
            |xs: &[f32], rvm: &mut RandomVarManager, mode: ProcessMode|
        {
            let w = F::parameter(&mut p_w);
            let b = F::parameter(&mut p_b);
            let xs = F::input(([], n_samples), xs);
            let mean = w * xs + b;
            let std = F::constant(([], 1), 0.1);
            // println!("{:?}", &ys);
            // println!("{:?}", (&mean).to_vector());
            let _ = rvm.process("y", &Normal::new(mean, std), mode);
        };

        // Inference loop
        for _i in 0..1000
        {
            g.clear();

            // Generative model
            let mut rvm = RandomVarManager::new();
            rvm.add_sample("y", F::input(([], n_samples), &ys));
            // model(&mut model, ProcessMode::SAMPLE);
            model(&xs, &mut rvm, ProcessMode::LOGP);

            // Objective function
            let nlogp = -F::batch::mean(&(rvm.logp));

            println!("nlogp = {:?}", nlogp.to_float());

            optimizer.reset_gradients();
            nlogp.backward();
            optimizer.update();
        }
    } // Brrowing p_mean and p_lstd ends here

    println!("(w, b) = ({}, {})", p_w.value().to_float(),
             p_b.value().to_float());
}
