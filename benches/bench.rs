use std::arch::x86_64::_lzcnt_u64;
use std::vec;
use std::{collections::HashMap, hash::Hash, collections::HashSet};

use criterion::{black_box, criterion_group, criterion_main, Criterion};


use rand::RngCore;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::rngs::SmallRng;

#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[inline]
fn better_fibonacci(n: u64) -> u64 {
  let mut table:HashMap<u64, u64> = HashMap::new();
  fn _better_fibo(n: u64, memo: &HashMap<u64, u64>)-> u64 {

    if memo.contains_key(&n){
      return *memo.get(&n).unwrap();
    }
    else {
      match n {
          0 => 1,
          1 => 1,
          n => _better_fibo(n - 1, memo) + fibonacci(n - 2)
      }
    }

  }
  return 0;
}


fn better_better_fibo(n: u64) -> u64 {
  let mut a = 0;
  let mut b = 1;

  match n {
      0 => b,
      _ => {
          for _ in 0..n {
              let c = a + b;
              a = b;
              b = c;
          }
          b
      }
  }
}





fn sorted_set_bench(mut s: HashSet<u32>, n:Vec<u32>){
  for x in n.iter() {
    s.insert(*x);
  }
}

use chess_game::types::*;
use chess_game::consts::*;
pub type BitBoard = u64;
use bitvec::{prelude::*, view::BitView};


fn branch_rook(){
  let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000000000000100000000000000;
  let white_mask:BitBoard =  0b0000000000000000000000000000000000000000000000001111111111111111;
  let black_mask:BitBoard = 0b0100000101000001000000000000000000000000000000000000000000000000;
  let mut res:u64 = 0;
  let mut shift:u8 = 8;
  let rook_init_pos = rook_mask.view_bits::<Msb0>().first_one().unwrap();
  while true {
    let rook_shift = (rook_mask << shift);
    res |= (rook_shift & !white_mask);
    if (res & black_mask) > 0 {
      break;
    }
    shift+=8;
  }
}

fn branchless_rook() {
  let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000000000000100000000000000;
  let white_mask:BitBoard =  0b0000000000000000000000000000000000000000000000001111111111111111;
  let black_mask:BitBoard = 0b0100000101000001000000000000000000000000000000000000000000000000;
  let mut toggle:u64 = 0;
  let mut res:u64 = 0;
  let a = (rook_mask << 8) & !white_mask;
  res |= a;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  let b = (rook_mask << 16) & !white_mask & a << 8 & !toggle as u64;
  res |= b;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  let c = (rook_mask << 24) & !white_mask & b << 8 & !toggle as u64;
  res |= c;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  let d = (rook_mask << 32) & !white_mask & c << 8 & !toggle as u64;
  res |= d;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  let e = (rook_mask << 40) & !white_mask & d << 8 & !toggle as u64;
  res |= e;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  let f = (rook_mask << 48) & !white_mask & e << 8 & !toggle as u64;
  res |= f;
  toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;

}




fn get_bit_idx_1(n:Vec<u64>){
  for x in n.iter(){
    unsafe {
      _lzcnt_u64(*x);
    }
  }
}

fn get_bit_idx_2(n: Vec<u64>){
  for x in n.iter() {
    x.view_bits::<Msb0>().first_one().unwrap();
  }
}

fn get_bit_idx_3(n: Vec<u64>){
  for x in n.iter(){
    x.leading_zeros();
  }
  
}

// fn get_bit_idx_4(n: Vec<u64>){
//   static DEBRUIJ_T: &'static [u8] = &[
//     0, 47,  1, 56, 48, 27,  2, 60,
//     57, 49, 41, 37, 28, 16,  3, 61,
//     54, 58, 35, 52, 50, 42, 21, 44,
//     38, 32, 29, 23, 17, 11,  4, 62,
//     46, 55, 26, 59, 40, 36, 15, 53,
//     34, 51, 20, 43, 31, 22, 10, 45,
//     25, 39, 14, 33, 19, 30,  9, 24,
//     13, 18,  8, 12,  7,  6,  5, 63
// ];

//   const DEBRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;
//   for x in n {
//     let bits = x + (x==0) as u64;
//     let idx = (((bits ^ bits.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58));
//     DEBRUIJ_T[idx as usize];
//   }
// }




pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("fib 20", |b| b.iter(|| better_fibonacci(black_box(20))));
    // c.bench_function("fib 20", |b| b.iter(|| better_better_fibo(black_box(20))));
    
    // let v_arr:[u32; 10e2 as usize] = [0; 10e2 as usize];
    // let mut v_rand = v_arr.map(|_| rng.next_u32());
    // v_rand.sort();
    // let mut v = Vec::from(v_rand);
    // let mut s = HashSet::from(v_rand);


    // let n_arr:[u32; 10 as usize] = [0; 10 as usize];
    // let n_rand = n_arr.map(|_| rng.next_u32());
    // let n = Vec::from(n_rand);



    // c.bench_function("branching", |b| b.iter(|| branch_rook()));
    // c.bench_function("branchless", |b| b.iter(|| branchless_rook()));
}

pub fn bit_ops_benchmarks(c: &mut Criterion){
  let mut rng = SmallRng::from_entropy();
  let n_arr: [u64; 10e3 as usize] = [0; 10e3 as usize];
  let n_rand = n_arr.map(|_| rng.next_u64());

  // c.bench_function("get_bit_idx_1", |b| b.iter(|| get_bit_idx_1(n_rand.to_vec())));
  // c.bench_function("get_bit_idx_2", |b| b.iter(|| get_bit_idx_2(n_rand.to_vec())));
  c.bench_function("get_bit_idx_3", |b| b.iter(|| get_bit_idx_3(n_rand.to_vec())));
  // c.bench_function("get_bit_idx_4", |b| b.iter(|| get_bit_idx_4(n_rand.to_vec())));
}





criterion_group!(benches, criterion_benchmark, bit_ops_benchmarks);
criterion_main!(benches);