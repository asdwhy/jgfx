use std::time::Instant;

use rand::{rngs::{SmallRng, StdRng}, SeedableRng, Rng};

use fastrand;

use xorshift::{Rng as XorRng, Xorshift128};
use xorshift::thread_rng;

fn main() {
    let num_trials: u128 = 10000000000;

    let now = Instant::now();
    let mut rng = SmallRng::from_entropy();
    for _ in 0..num_trials {
        let _ = rng.gen::<f64>();
    }
    println!("1 took : {}", now.elapsed().as_secs_f64());
}

