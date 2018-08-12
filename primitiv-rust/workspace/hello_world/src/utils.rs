extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

// Adapted from stack overflow
fn from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

pub fn new_rng(seed0: u8) -> StdRng {
    let seed: [u8; 32] =
        from_slice(&(seed0..(seed0 + 32)).collect::<Vec<_>>());
    let rng: StdRng = SeedableRng::from_seed(seed);
    rng
}

pub fn shuffle(n: usize, rng: &mut StdRng) -> Vec<usize> {
    let mut vec: Vec<usize> = (0..n).collect();
    {
        let slice = vec.as_mut_slice();
        rng.shuffle(slice);
    }
    vec
}

pub trait MiniBatchedDataset {
    type MiniBatch;

    fn take_minibatch(&self, ixs: &Vec<usize>) -> Self::MiniBatch;
    fn len(&self) -> usize;
}

pub struct MiniBatchIterator<'a, MB: 'a> {
    count: usize,
    minibatch_size: usize,
    ixs: Vec<usize>,
    dataset: &'a MiniBatchedDataset<MiniBatch=MB>,
}

impl<'a, MB> MiniBatchIterator<'a, MB> {
    pub fn new(minibatch_size: usize,
               dataset: &'a MiniBatchedDataset<MiniBatch=MB>, seed0: u8)
        -> Self
    {
        let n_samples = dataset.len();
        let mut rng = new_rng(seed0);
        let ixs = shuffle(n_samples, &mut rng);

        MiniBatchIterator {
            count: 0,
            minibatch_size: minibatch_size,
            ixs: ixs,
            dataset: dataset
        }
    }
}

impl<'a, MB> Iterator for MiniBatchIterator<'a, MB> {
    type Item = MB;

    fn next(&mut self) -> Option<MB> {
        if self.count < self.dataset.len() {
            let minibatch = self.dataset.take_minibatch(
                &self.ixs[self.count..self.count + self.minibatch_size].to_vec()
            );
            self.count += self.minibatch_size;

            Some(minibatch)

        } else {
            None
        }
    }
}
