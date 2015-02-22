//! Sketchy is a Rust library of probabilistic data structures.

#![feature(collections)]
#![feature(core)]
#![feature(hash)]

extern crate rand;

mod bloomfilter;
mod countmin;
mod hash;
mod hyperloglog;
mod reservoir;
mod topk;

pub use bloomfilter::BloomFilter;
pub use countmin::CountMinSketch;
pub use hyperloglog::HyperLogLog;
pub use reservoir::ReservoirSample;
pub use topk::TopK;
