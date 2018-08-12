extern crate hello_world;

use hello_world::{new_rng, shuffle};

struct MyMiniBatch {
    xs: Vec<f32>,
    ys: Vec<f32>,
}

struct MyDataSet<'a> {
    xs_all: &'a [f32],
    ys_all: &'a [f32],
}

impl<'a> MyDataSet<'a> {
    pub fn new(xs_all: &'a [f32], ys_all: &'a [f32]) -> Self {
        // TODO: assert len(xs_all) == len(ys_all)
        MyDataSet {
            xs_all: xs_all,
            ys_all: ys_all,
        }
    }
}

impl<'a> MiniBatchedDataset for MyDataSet<'a> {
    type MiniBatch = MyMiniBatch;

    fn take_minibatch(&self, ixs: &Vec<usize>) -> Self::MiniBatch {
        Self::MiniBatch {
            xs: ixs.clone().into_iter().
                map(|ix| self.xs_all[ix]).collect::<Vec<_>>(),
            ys: ixs.clone().into_iter().
                map(|ix| self.ys_all[ix]).collect::<Vec<_>>(),
        }
    }

    fn len(&self) -> usize {
        self.xs_all.len() as usize
    }
}

fn main() {
    println!("hoge");
}