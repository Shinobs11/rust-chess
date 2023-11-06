#![feature(portable_simd)]
#![feature(stdsimd)]


use std::arch::x86_64::{_mm512_and_epi64, _mm512_or_epi64};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
use Which::{First, Second};
use chesslib::cache::*;
use chesslib::chess::bit::*;
use chesslib::chess::consts::*;
use chesslib::chess::types::*;
use chesslib::chess::attack_bitmask::*;
use std::{iter::*, fmt};
use std::simd::*;
use std::fmt::*;
use bitvec::{prelude::*, view::BitView};
pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;

fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

  pub const default_pieces:[u8;32] = [
  60, //white king
  59, //white queen
  63, 56, //white rooks
  61, 58, //white bishops
  62, 57, //white knights
  47, 48, 49, 50, 51, 52, 53, 54, //white pawns
  4, //black king
  3, //black queen
  0, 7, //black rooks
  2, 5, //black bishops
  1, 6, //black knights
  8, 9, 10, 11, 12, 13, 14, 15 //black pawns
];



  // const W_KING_DEFAULT_MASK: u64 = (1 << (63 - 60));
  // const W_QUEEN_DEFAULT_MASK: u64 = (1 << (63 - 59));
  // const W_ROOK_DEFAULT_MASK: u64 = (1 << (63 - 63)) | (1 << (63 - 56));
  // const W_BISHOP_DEFAULT_MASK: u64 = (1 << (63 - 61)) | (1 << (63 - 58));
  // const W_KNIGHT_DEFAULT_MASK: u64 = (1 << (63 - 62)) | (1 << (63 - 57));
  // const W_PAWN_DEFAULT_MASK: u64 = (1 << (63 - 47)) | (1 << (63 - 48)) | (1 << (63 - 49)) | (1 << (63 - 50)) | (1 << (63 - 51)) | (1 << (63 - 52)) | (1 << (63 - 53)) | (1 << (63 - 54));

  // const B_KING_DEFAULT_MASK: u64 = (1 << (63 - 4));
  // const B_QUEEN_DEFAULT_MASK: u64 = (1 << (63 - 3));
  // const B_ROOK_DEFAULT_MASK: u64 = (1 << (63 - 0)) | (1 << (63 - 7));
  // const B_BISHOP_DEFAULT_MASK: u64 = (1 << (63 - 2)) | (1 << (63 - 5));
  // const B_KNIGHT_DEFAULT_MASK: u64 = (1 << (63 - 1)) | (1 << (63 - 6));
  // const B_PAWN_DEFAULT_MASK: u64 = (1 << (63 - 8)) | (1 << (63 - 9)) | (1 << (63 - 10)) | (1 << (63 - 11)) | (1 << (63 - 12)) | (1 << (63 - 13)) | (1 << (63 - 14)) | (1 << (63 - 15));

  // let rook_mask:BitBoard =      0b0000000000000000000000000000000000000000010000000000000000000000;
  // let bishop_mask:BitBoard =    0b0000000000000000000000000000000000001000000000000000000000000000;
  // println!("{}", bishop_mask.leading_zeros());
  // // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
  // let white_mask:BitBoard =     0b0000000000000000000000000000010000000000000000011111111111111111;
  // let mut black_mask:BitBoard =     0b0100000101000001000000000000000000000000000100000000000000000000;
  // black_mask |= bishop_mask << 17;
  // black_mask |= bishop_mask << 15;
  // black_mask |= bishop_mask << 10;
  // black_mask |= bishop_mask << 6;
  




  // // println!("{}", rook_mask << -8);
  
  // println!("rook_mask");
  // print_board(rook_mask);
  // println!("white_mask");
  // print_board(white_mask);
  // println!("black_mask");
  // print_board(black_mask);
  // println!("res");
  // print_board(rook_attack_mask(rook_mask, white_mask, black_mask));


  // let friend:u8 = 0b01100000;
  // let foe:u8 = 0b00000000;
  // let rook_mask:u8 = 0b00000001;
  // let rook_idx:u16 = rook_mask.leading_zeros() as u16;
  // let comb = (foe as u16) << 8 | friend as u16;
  // let res = RAY_CACHE[(rook_idx * u16::MAX) as usize + comb as usize];
  // // let rc = ray_cache();
  // println!("{:#010b}", res);

  // let mut n = 0;
  // for i in RAY_CACHE.iter() {
  //   if *i>0 {
  //     n+=1;
  //   }
  // }
  // println!("n: {}", n);
  // println!("res");
  // print_board(res);


  // let test_bb:u8 = 0xFA;
  // print_board(test_bb as u64);
  // print_board(put_col_mask(test_bb, 5));

    
    // let rook_idx: u8 = 2;
    // let friend_mask:u8 = 0b000000010;
    // let foe_mask:u8 = 0b00000010;
    // //0b0001000000000001
    // let m = get_ternary_bitmask(rook_idx, friend_mask, foe_mask);
    // println!("{:#010b}", RAY_CACHE[m as usize]);

  fn convert_bit_slice_to_u64(bs: &BitSlice)->u64{
    let mut res:u64 = 0;
    for (i, x) in bs.iter().enumerate(){
      res |= (1 << (63 - i)) & (((*x) as u64) * u64::MAX);
    }
    return res;
  }
  // let friend_mask_arr = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 1, 0, 1, 0, 0, 1, 0,
  //   1, 0, 1, 0, 1, 1, 0, 1,
  //   1, 1, 1, 1, 1, 0, 1, 1
  // ];
  // let foe_mask_arr = bits![
  //   1, 1, 1, 1, 1, 0, 1, 1,
  //   1, 1, 1, 0, 0, 1, 1, 1,
  //   0, 0, 0, 1, 0, 0, 1, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let bishop_mask_arr = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let expected_result = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 1, 0, 0, 0, 0, 0, 0,
  //   0, 0, 1, 0, 0, 0, 1, 0,
  //   0, 0, 0, 1, 0, 1, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 1, 0, 0,
  //   0, 0, 0, 0, 0, 0, 1, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let expected = convert_bit_slice_to_u64(expected_result);
  // println!("expected result: {}", board_to_string(expected));
  // let friend_mask = convert_bit_slice_to_u64(friend_mask_arr);
  // let foe_mask = convert_bit_slice_to_u64(foe_mask_arr);
  // let bishop_mask = convert_bit_slice_to_u64(bishop_mask_arr);

  // let b_attack_mask = bishop_attack_mask(bishop_mask, friend_mask, foe_mask);
  // println!("actual result: {}", board_to_string(b_attack_mask));
  // const GET_COL_MAGIC:u64 = 0x2040810204081u64;
  // let diag_num:u64  = convert_bit_slice_to_u64(diagonals);
  // let mask_num = convert_bit_slice_to_u64(mask);
  // let test_magic_num = convert_bit_slice_to_u64(test_magic);
  // println!("{:x}", test_magic_num);
  // let mut s = String::new();
  // for i in (0..8){
  //   let mut row_str = String::new();
  //   for j in (0..8){
  //     let d = (i*8 + j)/8;
  //     let d_str = d.to_string();

  //     row_str.push_str(d_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let m = (i*8 + j) % 8;
  //     let m_str = m.to_string();

  //     row_str.push_str(m_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let d = 7 - (i*8 + j)/8;
  //     let d_str = d.to_string();

  //     row_str.push_str(d_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let m = 7 - (i*8 + j) % 8;
  //     let m_str = m.to_string();

  //     row_str.push_str(m_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push('\n');
  //   s.push_str(row_str.as_str());
  // }
  // println!("{}", s);


  // // // let col_num = convert_bit_slice_to_u64(col);
  // let masked_diag_num = diag_num & mask_num;
  // print_mult_steps(masked_diag_num, test_magic_num);
  // println!();
  // print_board((masked_diag_num * test_magic_num) >> 56);
  // let mult_num = convert_bit_slice_to_u64(mult);
  // print_board(GET_COL_MAGIC);
  // print_board(diag_num);
  // println!();
  // print_board(mask_num);
  // println!();
  // print_board((diag_num & mask_num));
  // println!();
  // print_board((diag_num & mask_num) * 0xffedf9fd7cfcffff);

  // for (i, x) in DIAG_MASK_CACHE.iter().enumerate() {
  //   println!("{i}");
  //   print_board(*x);
  //   println!();
  // }
}


fn print_mult_steps(lhs: u64, rhs: u64) {
  let mut res:u64 = 0;
  for (i, x) in lhs.view_bits::<Lsb0>().iter().enumerate(){
    let toggle = (*x as u64) * u64::MAX;
    let mult = (toggle & rhs);
    if mult == 0{
      continue;
    }
    let shifted = (mult << i);
    
    let shifted_str = board_to_string(shifted);
    let prev_str = board_to_string(res);
    res += shifted;
    let post_str = board_to_string(res);

    let shifted_vec:Vec<&str> = shifted_str.split('\n').collect();
    let prev_vec:Vec<&str> = prev_str.split('\n').collect();
    
    let post_vec:Vec<&str> = post_str.split('\n').collect();
    println!("{}", i);

    for j in (0..8) {
      println!("{}  {}  {}",post_vec[j], prev_vec[j], shifted_vec[j]);

    }
  }    
}
fn board_to_string(b: BitBoard)->String{
  let slice = b.view_bits::<Msb0>();
  let mut s = String::new();
  for x in (0..8){
    for y in (0..8){
      s.push_str(format!("{:3?}", slice[x*8 + y] as u8).as_str());
    }
    s.push('\n');
  }
  return s;
}
fn print_board(b:BitBoard){
  let slice = b.view_bits::<Msb0>();
  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}