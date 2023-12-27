pub mod attack_bitmask;




use attack_bitmask::attack_bitmask_benchmark;
use criterion::{black_box, criterion_group, criterion_main, Criterion};






criterion_main!(attack_bitmask_benchmark);



