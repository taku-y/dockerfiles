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
    let n_samples = 5;
    let true_w = 2.5 as f32;
    let true_b = -1.0 as f32;
    let dist_x = NormalInRand::new(0.0, 10.0);
    let dist_e = NormalInRand::new(0.0, 0.1);
    let mut rng = rand::thread_rng();
    let xs = (0..n_samples).map(|_| dist_x.sample(&mut rng) as f32).collect::<Vec<_>>();
    let es = (0..n_samples).map(|_| dist_e.sample(&mut rng) as f32).collect::<Vec<_>>();
    let ys = xs.iter().zip(es.iter()).map(
        |(x, e)| true_w * x + true_b + e).collect::<Vec<_>>();

    println!("{:?}", xs);
    println!("{:?}", ys);
}
