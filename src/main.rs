#![feature(portable_simd)]
#![feature(stdsimd)]
mod util;

use std::arch::x86_64::{_mm512_and_epi64, _mm512_or_epi64};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
use Which::{First, Second};
use util::consts::*;
use util::chess::*;
use std::iter::*;
use crate::util::types::Board;
use std::simd::*;

pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;

fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  // let test = FENToBitBoard(s.to_string());
  // println!("{}", printBitBoard(test.w_pawn, false));
  

  

  // let mut bb = FullBitBoard{..Default::default()};
  
  // println!("{}", printBitBoard(bb.empty as u64, false));
  // setBit(&mut bb.empty, 63);
  // println!("{}", printBitBoard(bb.empty as u64, false));

  
  // let king_offsets = (
  //   (1, 1), (1, -1), (-1, 1), (-1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)
  // );
  // let mut rook_offsets_x: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x:i32| (x as i32, 0 as i32))).as_mut();
  // let mut rook_offsets_y: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x:i32|(0 as i32, x as i32))).as_mut();
  // rook_offsets_x.append(rook_offsets_y);
  // let rook_offsets = rook_offsets_x.clone();
  // let mut bishop_offsets_pos: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x|(x, x))).as_mut();
  // let mut bishop_offsets_neg: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x|(x, -x))).as_mut();
  // bishop_offsets_pos.append(bishop_offsets_neg);
  // let bishop_offsets = bishop_offsets_pos.clone();
  // let queen_offsets = Vec::new().append(rook_offsets.clone());

  // println!("{:?}", rook_offsets.0);




  // let b = Board::default();
  
  // let bb = Board::board_from_fen(s.to_string());
  

  // let arr = [0; 64];
  // println!("{}", b.to_string());
  // println!("{}", bb.to_string());

  fn simd_branchless_rook(rook_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{

    let rook_idx:u64 = rook_mask.leading_zeros() as u64;
    let mut result:u64x4 = u64x4::from_array([0;4]);
    let simd_friendly_pos_mask = u64x4::from_array([friendly_pos_mask; 4]);
    let simd_opponent_pos_mask = u64x4::from_array([opponent_pos_mask; 4]);
  
    let negative_offsets:u64x2 = u64x2::from_array([8, 1]);
    let positive_offsets:u64x2 = u64x2::from_array([8, 1]);
  
    let mut prev:u64x4 = u64x4::from_array([rook_mask;4]);
    let mut attack_toggle:u64x4 = u64x4::from_array([0;4]);

    let lb:u64x4 = u64x4::from([0, 0, rook_idx & !7, rook_idx & !7]);
    let ub:u64x4 = u64x4::from([63, 63, rook_idx | 7, rook_idx | 7]);
    let simd_rook_idx = u64x4::from_array([rook_idx;4]);
    for i in (1..8){
      let simd_i:u64x4 = u64x4::from([i;4]);
      let has_been_blocked:u64x4 = prev.simd_ne(u64x4::from_array([0;4])).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]);
      let left_shift = u64x2::from_array([rook_mask;2]) << (simd_swizzle!(simd_i, [0, 2]) * negative_offsets);
      let right_shift = u64x2::from_array([rook_mask;2]) >> (simd_swizzle!(simd_i, [1, 3]) * positive_offsets);
      let shift:Simd<u64, 4> = simd_swizzle!(left_shift, right_shift, [First(0), Second(0), First(1), Second(1)]);
      let offsets:i64x4 = simd_i.cast() * i64x4::from_array([-8, 8, -1, 1]) + simd_rook_idx.cast();
      let checked_shift:Simd<u64, 4> = ((offsets.simd_ge(lb.cast()) & offsets.simd_le(ub.cast())).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4])) & shift;
      prev = has_been_blocked & checked_shift & !simd_friendly_pos_mask & !attack_toggle;
      attack_toggle = ((prev & simd_opponent_pos_mask).simd_gt(u64x4::from_array([0; 4])).to_int().abs().cast() * u64x4::from_array([u64::MAX;4]));
      result |= prev;
    }
    return result.reduce_or();
  }

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
  

  


  // let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000000000000100000000000000;
  let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000010000000000000000000000;
  // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
  let white_mask:BitBoard =  0b0000000000000000000000000000000000000000000000001111111111111111;
  let black_mask:BitBoard = 0b0100000101000001000000000000000000000000000100000000000000000000;
  


  // println!("{}", rook_mask << -8);
  
  println!("rook_mask");
  print_board(rook_mask);
  println!("white_mask");
  print_board(white_mask);
  println!("black_mask");
  print_board(black_mask);
  println!("res");
  print_board(simd_branchless_rook(rook_mask, white_mask, black_mask));


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
  // println!("{}", masks.len());
  // let res = branchless_rook(rook_mask, white_mask, black_mask);


  // println!("res");
  // print_board(res);


    
}

use bitvec::{prelude::*, view::BitView};

fn print_board(b:BitBoard){
  let slice = b.view_bits::<Msb0>();

  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}