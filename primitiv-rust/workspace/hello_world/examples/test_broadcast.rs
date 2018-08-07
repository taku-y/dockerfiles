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

fn broadcast1() {
    let a = F::input(([4]), &vec![1.0, 2.0, 3.0, 4.0]);
    let b = F::input(([], 5), &vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    let c = a * b;
    println!("{:?}", c.shape().dims());
    println!("{:?}", c.shape().batch());
    println!("{:?}", c.to_vector());
}

fn broadcast2() {
    let a = F::input(([1, 4]), &vec![1.0, 2.0, 3.0, 4.0]);
    let b = F::input(([3, 1]), &vec![10.0, 20.0, 30.0]);
    let c = a + b;
    println!("{:?}", c.shape().dims());
    println!("{:?}", c.shape().batch());
    println!("{:?}", c.to_vector());
}

fn main() {
    let mut dev = D::Naive::new();
    D::set_default(&mut dev);
    let mut g = Graph::new();
    Graph::set_default(&mut g);

    broadcast1();
    broadcast2();
}
