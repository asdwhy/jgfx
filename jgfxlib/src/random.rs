// use std::cell::UnsafeCell;

// use xorshift::{Rng, thread_rng, Xoroshiro128};

// thread_local! {
//     static RNG2: UnsafeCell<Xoroshiro128> = UnsafeCell::new(thread_rng());
// }

// // /// Returns a random f64 in [0,1)
// // pub fn random_f64() -> f64 {
// //     RNG2.with(|rng| {
// //         unsafe {
// //             (*rng.get()).next_f64()
// //         }
// //     })
// // }

// // /// Returns a random f64 in [min,max)
// // pub fn random_f64_range(min: f64, max: f64) -> f64 {
// //     min + (max-min)*random_f64()
// // }



