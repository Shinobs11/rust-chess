use std::arch::x86_64::{_lzcnt_u64};
use std::{collections::HashMap, hash::Hash, collections::HashSet};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::Distribution;
use rand::{RngCore, distributions};
use rand::distributions::Uniform;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use chess_game::util::chess::*;
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


fn branching_rook(rook_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64){
  const FILE_DELTAS:&'static[i32; 2] = &[-8, 8];
  const RANK_DELTAS:&'static[i32; 2] = &[-1, 1];
  let rook_idx = rook_mask.leading_zeros();
  let file_bounds:(i32, i32) = (0, 63);
  let rank_bounds:(i32, i32) = ((rook_idx & !7) as i32, (rook_idx | 7) as i32);
  let mut result:u64 = 0;
  fn left_shift(value:u64, shift:u64)->u64{
    return value.shl(shift);
  }
  fn right_shift(value: u64, shift: u64)->u64{
    return value.shr(shift);
  }
  for delta in FILE_DELTAS.iter(){
    //potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.
    let mut offset_results:[u64; 8] = [rook_mask, 0, 0, 0, 0, 0, 0, 0];
    let mut attack_toggle:bool = false;
    let op = if (*delta < 0) {left_shift} else {right_shift};
    let shift_sign = delta.signum();
    for i in (1..8){
      let idx = (rook_idx as i32 + i*delta);
      let in_bounds = (idx >= file_bounds.0 &&  idx <= file_bounds.1);
      let has_been_blocked = !(offset_results[(i-1) as usize] == 0);
      if has_been_blocked || !in_bounds || attack_toggle {
        continue;
      }
      offset_results[i as usize] = (op(rook_mask, (shift_sign * (i as i32 * *delta)) as u64)) & !friendly_pos_mask;
      attack_toggle = ((offset_results[i as usize] & opponent_pos_mask) > 0);  
    }
    for i in (1..8) {
      result |= offset_results[i];
    }
  }
  for delta in RANK_DELTAS.iter(){
    // potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.

    let mut offset_results:[u64; 8] = [rook_mask, 0, 0, 0, 0, 0, 0, 0];
    let mut attack_toggle:bool = false;
    let op = if (*delta < 0) {left_shift} else {right_shift};
    let shift_sign = delta.signum();
    for i in (1..8){
      let idx = (rook_idx as i32 + i*delta);
      let in_bounds = (idx >= rank_bounds.0 &&  idx <= rank_bounds.1);
      let has_been_blocked = !(offset_results[(i-1) as usize] == 0);
      if has_been_blocked || !in_bounds || attack_toggle {
        continue;
      }
      offset_results[i as usize] = (op(rook_mask, (shift_sign * (i as i32 * *delta)) as u64)) & !friendly_pos_mask;
      attack_toggle = ((offset_results[i as usize] & opponent_pos_mask) > 0);  
    }
    for i in (1..8) {
      result |= offset_results[i];
    }
  }

}
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
pub fn branchless_rook(rook_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let rook_idx = rook_mask.leading_zeros();
  let mut result:u64 = 0;

  //potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.
  // delta = -8
  let mut prev:u64 = rook_mask;
  let mut attack_toggle:u64 = 0;
  let mut range_max:u32 = rook_idx/8;
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask << (i*8)) & !friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 8
  
  prev = rook_mask;
  attack_toggle = 0;
  range_max = (63-rook_idx)/8;
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask >> (i*8)) & !friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = -1
  prev = rook_mask;
  attack_toggle = 0;
  range_max = (rook_idx % 8);
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask << (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 1
  prev = rook_mask;
  attack_toggle = 0;
  range_max = 7 - (rook_idx % 8);
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask >> (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }
  
  return result;
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



fn batch_branchless_rook(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (rook_mask, friend_mask, foe_mask) in v {
    res.push(branchless_rook(*rook_mask, *friend_mask, *foe_mask));
  }
  return res;
}

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


    let fens = retrieve_fens("/home/shino/chess-datasets/1000-rook-positions.fen".to_string());
    let boards:Vec<Board> = parse_fens(fens);

    let mut masks:Vec<(u64, u64, u64)> = Vec::<(u64, u64, u64)>::with_capacity(boards.len());
    for (i, b) in (&boards).iter().enumerate() {

      let piece_mask:u64;
      let friend_mask:u64;
      let opp_mask:u64;

      
      friend_mask = if b.turn == 0 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
      opp_mask = if b.turn == 1 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
      
      if b.turn == 0 {
        if b.piece_set[Piece::WRook].is_empty() {
          continue;
        } else {
          let idx = *b.piece_set[Piece::WRook].iter().next().unwrap();
          piece_mask = (1u64 << idx);
        }
      }
      else if b.turn == 1 {
        if b.piece_set[Piece::BRook].is_empty(){
          continue;
        } else {
          let idx = *b.piece_set[Piece::BRook].iter().next().unwrap();
          piece_mask = (1u64 << idx);
        }
      }
      else {
        panic!("???");
      }
      masks.push((piece_mask, friend_mask, opp_mask));
    }






    // rook_mask = op(rook_mask, offset as u64);

    // c.bench_function("branching", |b| b.iter(|| branching_rook(rook_mask, white_mask, black_mask)));
    // c.bench_function("branchless", |b| b.iter(|| branchless_rook(rook_mask, white_mask, black_mask)));
    c.bench_function("batch_branchless", |b| b.iter(|| batch_branchless_rook(&masks)));

  }

pub fn bit_ops_benchmarks(c: &mut Criterion){
  let mut rng = SmallRng::from_entropy();
  let n_arr: [u64; 10e3 as usize] = [0; 10e3 as usize];
  let n_rand = n_arr.map(|_| rng.next_u64());

  // c.bench_function("get_bit_idx_1", |b| b.iter(|| get_bit_idx_1(n_rand.to_vec())));
  // c.bench_function("get_bit_idx_2", |b| b.iter(|| get_bit_idx_2(n_rand.to_vec())));
  // c.bench_function("get_bit_idx_3", |b| b.iter(|| get_bit_idx_3(n_rand.to_vec())));
  // c.bench_function("get_bit_idx_4", |b| b.iter(|| get_bit_idx_4(n_rand.to_vec())));
}





criterion_group!(benches, criterion_benchmark, bit_ops_benchmarks);
criterion_main!(benches);