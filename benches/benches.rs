// extern crate criterion;
use criterion::criterion_main;
mod precompute;
mod empty_map;
mod two_percent;

criterion_main!{
    // precompute::precompute,
    // empty_map::empty_map,
    two_percent::two_percent,
}