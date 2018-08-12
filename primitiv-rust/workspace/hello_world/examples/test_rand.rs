//extern crate rand;
extern crate hello_world;

//use rand::Rng;
use hello_world::utils::{new_rng, shuffle};

fn main() {
    let mut rng = new_rng(0);
    //println!("{}", rng.gen::<f64>());
    let ixs = shuffle(13, &mut rng);
    println!("{:?}", ixs);
}
