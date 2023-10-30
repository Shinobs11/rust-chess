#![feature(portable_simd)]
#![feature(stdsimd)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_enum::FromPrimitive;
use rand::Rng;
use rand::{RngCore, distributions};
use Which::{First, Second};
use std::simd::*;
use bitvec::{prelude::*, view::BitView};
use chesslib::chess::consts::*;
use chesslib::chess::types::*;
use chesslib::chess::attack_bitmask::*;
use chesslib::chess::chess::*;
pub type BitBoard = u64;


pub fn criterion_benchmark(c: &mut Criterion) {
    
    fn get_position_masks(path: &str, target_piece: GenericPiece)->Vec<(u64, u64, u64)> {
      let fens = retrieve_fens(path.to_string());
      let boards = parse_fens(fens);

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
    let mut knight_masks = get_position_masks("/home/shino/chess-datasets/1000-N-positions.fen", GenericPiece::Knight);
    let mut bishop_masks = get_position_masks("/home/shino/chess-datasets/1000-B-positions.fen", GenericPiece::Bishop);
    let mut rook_masks = get_position_masks("/home/shino/chess-datasets/1000-R-positions.fen", GenericPiece::Rook);
    let mut queen_masks = get_position_masks("/home/shino/chess-datasets/1000-Q-positions.fen", GenericPiece::Queen);
    
    let mut combined_masks = Vec::<(u64, u64, u64, GenericPiece)>::new();
    
    let mut knight_masks_copy = Vec::<(u64, u64, u64)>::new();
    knight_masks_copy.clone_from(&knight_masks);
    let mut bishop_masks_copy = Vec::<(u64, u64, u64)>::new();
    bishop_masks_copy.clone_from(&bishop_masks);
    let mut rook_masks_copy = Vec::<(u64, u64, u64)>::new();
    rook_masks_copy.clone_from(&rook_masks);
    let mut queen_masks_copy = Vec::<(u64, u64, u64)>::new();
    queen_masks_copy.clone_from(&queen_masks);


    
    let mut rng = rand::thread_rng();
    let mut  n = 0;
    for _ in 0..(bishop_masks_copy.len() + knight_masks_copy.len() + rook_masks_copy.len() + queen_masks_copy.len()) {
      let n_piece = GenericPiece::from_primitive(n);
      match n_piece {
        GenericPiece::Knight => {
          if knight_masks_copy.is_empty() {
            continue;
          }
          else {
            let x = knight_masks_copy.pop().unwrap();
            combined_masks.push((x.0, x.1, x.2, GenericPiece::Knight));
          }
        },
        GenericPiece::Bishop => {
          if bishop_masks_copy.is_empty() {
            continue;
          }
          else {
            let x = bishop_masks_copy.pop().unwrap();
            combined_masks.push((x.0, x.1, x.2, GenericPiece::Bishop));
          }
        },
        GenericPiece::Rook => {
          if rook_masks_copy.is_empty() {
            continue;
          }
          else {
            let x = rook_masks_copy.pop().unwrap();
            combined_masks.push((x.0, x.1, x.2, GenericPiece::Rook));
          }
        },
        GenericPiece::Queen => {
          if queen_masks_copy.is_empty() {
            continue;
          }
          else {
            let x = queen_masks_copy.pop().unwrap();
            combined_masks.push((x.0, x.1, x.2, GenericPiece::Queen));
          }
        },
        GenericPiece::King => (),
        GenericPiece::Pawn => (),
        GenericPiece::Empty => ()
      }
      n += 1;
      if n >= 4 {
        n = 0;
      }
    }

    // rook_mask = op(rook_mask, offset as u64);

    // c.bench_function("branching", |b| b.iter(|| branching_rook(rook_mask, white_mask, black_mask)));
    // c.bench_function("branchless", |b| b.iter(|| branchless_rook(rook_mask, white_mask, black_mask)));
    
    // c.bench_function("batch_branchless_bishop", |b| b.iter(|| batch_bishop_attack_mask(&masks)));
    // c.bench_function("simd_batch_branchless", |b| b.iter(|| batch_simd_branchless_rook(&masks)));

    c.bench_function("batch_knight_attack_mask", |b| b.iter(|| batch_knight_attack_mask(&knight_masks)));
    c.bench_function("batch_bishop_attack_mask", |b| b.iter(|| batch_bishop_attack_mask(&bishop_masks)));
    c.bench_function("batch_rook_attack_mask", |b| b.iter(|| batch_rook_attack_mask(&rook_masks)));
    c.bench_function("batch_queen_attack_mask", |b| b.iter(|| batch_queen_attack_mask(&queen_masks)));
    c.bench_function("batch_all_attack_mask", |b| b.iter(|| batch_all_attack_mask(&combined_masks)));
  }







criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);