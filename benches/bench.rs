#![feature(portable_simd)]
#![feature(stdsimd)]
use std::arch::x86_64::{_lzcnt_u64};
use std::{collections::HashMap, hash::Hash, collections::HashSet};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_enum::FromPrimitive;
use rand::prelude::Distribution;
use rand::{RngCore, distributions};
use rand::distributions::Uniform;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use chess_game::util::chess::*;
use chess_game::exp::attack_bitmask::*;
use core::simd;
use Which::{First, Second};
use std::simd::*;
use bitvec::{prelude::*, view::BitView};
use chess_game::types::*;
use chess_game::consts::*;
pub type BitBoard = u64;








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


    // let fens = retrieve_fens("/home/shino/chess-datasets/1000-rook-positions.fen".to_string());
    // let boards:Vec<Board> = parse_fens(fens);

    // let mut masks:Vec<(u64, u64, u64)> = Vec::<(u64, u64, u64)>::with_capacity(boards.len());
    // for (i, b) in (&boards).iter().enumerate() {

    //   let piece_mask:u64;
    //   let friend_mask:u64;
    //   let opp_mask:u64;

      
    //   friend_mask = if b.turn == 0 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
    //   opp_mask = if b.turn == 1 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
      
    //   if b.turn == 0 {
    //     if b.piece_set[Piece::WRook].is_empty() {
    //       continue;
    //     } else {
    //       let idx = *b.piece_set[Piece::WRook].iter().next().unwrap();
    //       piece_mask = (1u64 << idx);
    //     }
    //   }
    //   else if b.turn == 1 {
    //     if b.piece_set[Piece::BRook].is_empty(){
    //       continue;
    //     } else {
    //       let idx = *b.piece_set[Piece::BRook].iter().next().unwrap();
    //       piece_mask = (1u64 << idx);
    //     }
    //   }
    //   else {
    //     panic!("???");
    //   }
    //   masks.push((piece_mask, friend_mask, opp_mask));
    // }



    fn get_position_masks(boards: &Vec<Board>, target_piece: GenericPiece)->Vec<(u64, u64, u64)> {
      let mut masks:Vec<(u64, u64, u64)> = Vec::<(u64, u64, u64)>::with_capacity(boards.len());
      for (i, b) in (&boards).iter().enumerate() {

        let piece_mask:u64;
        let friend_mask:u64;
        let opp_mask:u64;

        
        friend_mask = if b.turn == 0 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
        opp_mask = if b.turn == 1 {b.get_piece_mask(0)} else {b.get_piece_mask(1)};
        let p = (target_piece as u8) * 2 + b.turn;
        let idx = b.piece_set[Piece::from_primitive(p)].iter().next().unwrap();
        piece_mask = (1u64 << idx);
        masks.push((piece_mask, friend_mask, opp_mask));
      }
      return masks;
    }


    // let fens = retrieve_fens("/home/shino/chess-datasets/1000-Q-positions.fen".to_string());
    // let boards:Vec<Board> = parse_fens(fens);
    // let queen_masks = get_position_masks(&boards, GenericPiece::Queen);
    let fens = retrieve_fens("/home/shino/chess-datasets/1000-R-positions.fen".to_string());
    let boards:Vec<Board> = parse_fens(fens);
    let rook_masks = get_position_masks(&boards, GenericPiece::Rook);

    let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000010000000000000000000000;
    // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
    let white_mask:BitBoard =  0b0000000000000000000000000000000000000000000000001111111111111111;
    let black_mask:BitBoard = 0b0100000101000001000000000000000000000000000100000000000000000000;



    // rook_mask = op(rook_mask, offset as u64);

    // c.bench_function("branching", |b| b.iter(|| branching_rook(rook_mask, white_mask, black_mask)));
    // c.bench_function("branchless", |b| b.iter(|| branchless_rook(rook_mask, white_mask, black_mask)));
    
    // c.bench_function("batch_branchless_bishop", |b| b.iter(|| batch_bishop_attack_mask(&masks)));
    // c.bench_function("simd_batch_branchless", |b| b.iter(|| batch_simd_branchless_rook(&masks)));
    c.bench_function("batch_rook_attack_mask", |b| b.iter(|| batch_rook_attack_mask(&rook_masks)));
    // c.bench_function("batch_queen_attack_mask", |b| b.iter(|| batch_queen_attack_mask(&queen_masks)));
    // c.bench_function("batch_knight_attack_mask", |b| b.iter(|| batch_knight_attack_mask(&queen_masks)));
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