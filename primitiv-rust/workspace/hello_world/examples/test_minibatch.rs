extern crate hello_world;

use hello_world::utils::{MiniBatchedDataset, MiniBatchIterator};

#[derive(Debug)]
struct MyMiniBatch {
    xs: Vec<f32>,
    ys: Vec<f32>,
}

struct MyDataSet<'a> {
    xs_all: &'a [f32],
    ys_all: &'a [f32],
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
    println!("Create a dataset");
    let xs = (0..100).map(|x| x as f32).collect::<Vec<_>>();
    let ys = (100..200).map(|x| x as f32).collect::<Vec<_>>();
    let dataset = MyDataSet {
        xs_all: &xs,
        ys_all: &ys
    };
    let iter = MiniBatchIterator::new(5, &dataset, 0);

    for mb in iter {
        println!("{:?}", mb);
    }
}
