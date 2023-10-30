
#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#[deny(long_running_const_eval)]

pub mod chess;
pub mod chess_base;
pub mod bit;
pub mod consts;
pub mod types;
pub mod attack_bitmask;
